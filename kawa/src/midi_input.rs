use crate::elements_handlers::*;
use midi_port::*;

pub struct MidiInput<MidiUart>
where
    MidiUart: embedded_hal::serial::Read<u8>,
{
    port: MidiInPort<MidiUart>,
}

impl<MidiUart> MidiInput<MidiUart>
where
    MidiUart: embedded_hal::serial::Read<u8>,
{
    pub fn new(port: MidiInPort<MidiUart>) -> Self {
        MidiInput { port }
    }

    pub fn handle_midi_irq(&mut self) {
        self.port.poll_uart();

        if let Some(message) = self.port.get_message() {
            match message {
                MidiMessage::NoteOn {
                    channel: _,
                    note,
                    velocity,
                } => self.handle_note(true, note, velocity),
                MidiMessage::NoteOff {
                    channel: _,
                    note,
                    velocity,
                } => self.handle_note(false, note, velocity),
                MidiMessage::Aftertouch {
                    channel: _,
                    note: None,
                    value,
                } => self.set_modulation(value),
                _ => (),
            };
        }
    }

    fn handle_note(&mut self, on: bool, note: NoteNumber, velocity: u8) {
        unsafe {
            Elements_SetGate(on);
            if on {
                Elements_SetNote(note as f32);
                Elements_SetStrength((velocity as f32) / 127.0);
                Elements_SetModulation(0.0);
            }
        }
    }

    fn set_modulation(&mut self, value: u8) {
        unsafe {
            Elements_SetModulation((value as f32) / 127.0);
        }
    }
}
