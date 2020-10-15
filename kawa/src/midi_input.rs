use crate::synth::VoiceEventsQueue;
use crate::voice::VoiceEvent;
use alloc::vec::Vec;
use midi_port::*;

pub struct MidiInput<MidiUart>
where
    MidiUart: embedded_hal::serial::Read<u8>,
{
    port: MidiInPort<MidiUart>,
    notes_stack: Vec<Note>,
    legato: bool,
}

struct Note {
    note: NoteNumber,
    velocity: u8,
}

const NOTE_STACK_SIZE: usize = 5;

impl<MidiUart> MidiInput<MidiUart>
where
    MidiUart: embedded_hal::serial::Read<u8>,
{
    pub fn new(port: MidiInPort<MidiUart>) -> Self {
        MidiInput {
            port,
            notes_stack: Vec::with_capacity(NOTE_STACK_SIZE),
            legato: false,
        }
    }

    pub fn handle_midi_irq(&mut self, events: &mut VoiceEventsQueue) {
        self.port.poll_uart();

        if let Some(message) = self.port.get_message() {
            let event = match message {
                MidiMessage::NoteOn {
                    channel: _,
                    note,
                    velocity,
                } => Some(self.handle_note(true, note, velocity)),
                MidiMessage::NoteOff {
                    channel: _,
                    note,
                    velocity,
                } => Some(self.handle_note(false, note, velocity)),
                MidiMessage::Aftertouch {
                    channel: _,
                    note: None,
                    value,
                } => Some(VoiceEvent::ChangeStrength((value as f32) / 127.0)),
                MidiMessage::PitchBendChange { channel: _, value } => {
                    Some(VoiceEvent::ChangePitchModulation(
                        (value as f32 - 8192 as f32) * 2f32 / 8192 as f32,
                    ))
                }
                _ => None,
            };

            if let Some(event) = event {
                events.enque(event);
            }
        }
    }

    fn handle_note(&mut self, on: bool, note: NoteNumber, velocity: u8) -> VoiceEvent {
        if on {
            if self.notes_stack.len() == NOTE_STACK_SIZE {
                self.notes_stack.remove(0);
            }
            self.notes_stack.push(Note { note, velocity });
        } else {
            self.notes_stack.drain_filter(|n| n.note == note);
        }

        if self.notes_stack.len() > 0 {
            let note = &self.notes_stack[self.notes_stack.len() - 1];

            return VoiceEvent::NoteOn {
                retrigger: !self.legato,
                note: note.note as f32,
                strength: (note.velocity as f32) / 127.0,
            };
        } else {
            return VoiceEvent::NoteOff;
        }
    }
}
