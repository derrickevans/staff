use svg::{
    node::element::{Ellipse, Line},
    Document, Node,
};

use crate::{
    midi::{MidiNote, MidiSet, Octave},
    Pitch,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Eigth,
    Quarter,
    Half,
    Whole,
}

pub struct Measure {
    chords: Vec<Chord>,
}

const NOTE_RX: i64 = 8;
const NOTE_RY: i64 = 6;

impl Measure {
    pub fn f(&self, doc: &mut Document, x: i64) {
        let mut chord_x = x + 28;

        let mut pos = 0;
        while let Some(chord) = self.chords.get(pos) {
            chord_x += match chord.duration {
                Duration::Whole => {
                    chord.write_svg(doc, chord_x);
                    200
                }
                Duration::Half => {
                    chord.write_svg(doc, chord_x);
                    chord.draw_note_line(doc, chord_x, 40);
                    200 / 2
                }
                Duration::Quarter => {
                    chord.write_svg(doc, chord_x);
                    chord.draw_note_line(doc, chord_x, 40);
                    200 / 4
                }
                Duration::Eigth => {
                    chord.write_svg(doc, chord_x);

                    let low = *chord.notes.iter().min().unwrap();
                    let high = *chord.notes.iter().max().unwrap();
                    let diff = low.max(10 - high);

                    if let Some(next) = self.chords.get(pos + 1) {
                        if next.duration == Duration::Eigth {
                            let low_next = *next.notes.iter().min().unwrap();
                            let high_next = *next.notes.iter().max().unwrap();
                            let diff_next = low_next.max(10 - high_next);

                            let (x, y1, y2, y1_next, y2_next, y) = if diff > diff_next {
                                if low > 10 - high {
                                    (
                                        chord_x,
                                        y(low) + 40,
                                        y(high),
                                        y(low) + 40,
                                        y(high_next),
                                        y(low) + 40,
                                    )
                                } else {
                                    (
                                        chord_x + 8,
                                        y(low),
                                        y(high_next) - 40,
                                        y(low_next),
                                        y(high_next) - 40,
                                        y(high_next) - 40 + 4,
                                    )
                                }
                            } else {
                                if low_next > 10 - high_next {
                                    (
                                        chord_x,
                                        y(low) + 40,
                                        y(high_next),
                                        y(low_next) + 40,
                                        y(high_next),
                                        y(high_next),
                                    )
                                } else {
                                    (
                                        chord_x - 8,
                                        y(high),
                                        y(low_next) + 40,
                                        y(high_next),
                                        y(low_next) + 40,
                                        y(low_next) + 40 - 4,
                                    )
                                }
                            };

                            doc.append(
                                Line::new()
                                    .set("fill", "none")
                                    .set("stroke", "black")
                                    .set("stroke-width", 2)
                                    .set("x1", x)
                                    .set("y1", y1)
                                    .set("x2", x)
                                    .set("y2", y2),
                            );

                            const OFFSET: i64 = 30;
                            let next_x = x + OFFSET;
                            doc.append(
                                Line::new()
                                    .set("fill", "none")
                                    .set("stroke", "black")
                                    .set("stroke-width", 2)
                                    .set("x1", next_x)
                                    .set("y1", y1_next)
                                    .set("x2", next_x)
                                    .set("y2", y2_next),
                            );

                            doc.append(
                                Line::new()
                                    .set("fill", "none")
                                    .set("stroke", "black")
                                    .set("stroke-width", 8)
                                    .set("x1", x)
                                    .set("y1", y)
                                    .set("x2", next_x)
                                    .set("y2", y),
                            );

                            next.write_svg(doc, chord_x + OFFSET);

                            pos += 1;
                            70
                        } else {
                            20
                        }
                    } else {
                        20
                    }
                }
            };

            pos += 1;
        }

        for line in 0..5 {
            let y = line * NOTE_RY * 2 + 50;

            doc.append(
                Line::new()
                    .set("x1", x)
                    .set("y1", y)
                    .set("x2", x + chord_x)
                    .set("y2", y)
                    .set("stroke", "#000")
                    .set("stroke-width", 2),
            )
        }

        for line in 0..2 {
            let line_x = line * chord_x + x;

            doc.append(
                Line::new()
                    .set("x1", line_x)
                    .set("y1", 50)
                    .set("x2", line_x)
                    .set("y2", 98)
                    .set("stroke", "#000"),
            )
        }
    }
}

pub struct Chord {
    // TODO use Set
    notes: Vec<i64>,
    duration: Duration,
}

fn y(note: i64) -> i64 {
    (17 - note) * NOTE_RY - 4
}

fn note_head(x: i64, note: i64) -> Ellipse {
    let cx = x;
    let cy = y(note);

    Ellipse::new()
        .set("cx", cx)
        .set("cy", cy)
        .set("rx", NOTE_RX)
        .set("ry", NOTE_RY)
}

impl Chord {
    pub fn write_svg(&self, doc: &mut Document, x: i64) {
        match self.duration {
            Duration::Whole => {
                for note in &self.notes {
                    doc.append(
                        Ellipse::new()
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("cx", x + 10)
                            .set("cy", y(*note))
                            .set("rx", 10)
                            .set("ry", 5),
                    );
                }
            }
            Duration::Half => {
                for note in &self.notes {
                    doc.append(
                        note_head(x, *note)
                            .set("fill", "none")
                            .set("stroke", "black"),
                    );
                }
            }
            Duration::Quarter => {
                for note in &self.notes {
                    doc.append(note_head(x, *note).set("fill", "black"));
                }
            }
            Duration::Eigth => {
                for note in &self.notes {
                    if note & 1 == 0 && self.notes.contains(&(note + 1)) {
                        doc.append(
                            note_head(x + (NOTE_RX * 2), *note)
                                .set("fill", "black")
                                .set("wat", *note),
                        );
                    } else {
                        doc.append(note_head(x, *note).set("fill", "black").set("wat", *note));
                    }
                }
            }
        }
    }

    fn draw_note_line(&self, doc: &mut Document, x: i64, extra: i64) -> i64 {
        let low = *self.notes.iter().min().unwrap();
        let high = *self.notes.iter().max().unwrap();

        if low > 10 - high {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", x)
                    .set("y1", y(low) + extra)
                    .set("x2", x)
                    .set("y2", y(high)),
            );
            y(low) + 40
        } else {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", x + NOTE_RX)
                    .set("y1", y(low))
                    .set("x2", x + NOTE_RX)
                    .set("y2", y(high) - extra),
            );
            y(high) - 40
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Chord, Duration, Measure};

    #[test]
    fn f() {
        let measure = Measure {
            chords: vec![
                Chord {
                    notes: vec![5, 7, 9],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![5],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-2],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-2, 0, 2],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-3],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-3, -2, -1],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![4],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![5, 4, 3],
                    duration: Duration::Eigth,
                },
            ],
        };

        let mut document = svg::Document::new();
        measure.f(&mut document, 10);

        svg::save("image.svg", &document).unwrap();
    }
}