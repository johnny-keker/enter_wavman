/* Useful type aliases */
pub type Scale = [f32; 7];
pub type Lead = [&'static Scale; 3];
pub type Chord = [f32; 3];
pub type ChordPool = [&'static Chord; 7];
pub struct Key {
  pub lead: Lead,
  pub chords: ChordPool,
}

/* Single notes */
pub const C0: f32 = 16.35;
pub const C_SH_0: f32 = 17.32;
pub const D0: f32 = 18.35;
pub const D_SH_0: f32 = 19.45;
pub const E0: f32 = 20.6;
pub const F0: f32 = 21.83;
pub const F_SH_0: f32 = 23.12;
pub const G0: f32 = 24.5;
pub const G_SH_0: f32 = 25.96;
pub const A0: f32 = 27.5;
pub const A_SH_0: f32 = 29.14;
pub const B0: f32 = 30.87;
pub const C1: f32 = 32.7;
pub const C_SH_1: f32 = 34.65;
pub const D1: f32 = 36.71;
pub const D_SH_1: f32 = 38.89;
pub const E1: f32 = 41.2;
pub const F1: f32 = 43.65;
pub const F_SH_1: f32 = 46.25;
pub const G1: f32 = 49.0;
pub const G_SH_1: f32 = 51.91;
pub const A1: f32 = 55.0;
pub const A_SH_1: f32 = 58.27;
pub const B1: f32 = 61.74;
pub const C2: f32 = 65.41;
pub const C_SH_2: f32 = 69.3;
pub const D2: f32 = 73.42;
pub const D_SH_2: f32 = 77.78;
pub const E2: f32 = 82.41;
pub const F2: f32 = 87.31;
pub const F_SH_2: f32 = 92.5;
pub const G2: f32 = 98.0;
pub const G_SH_2: f32 = 103.83;
pub const A2: f32 = 110.0;
pub const A_SH_2: f32 = 116.54;
pub const B2: f32 = 123.47;
pub const C3: f32 = 130.81;
pub const C_SH_3: f32 = 138.59;
pub const D3: f32 = 146.83;
pub const D_SH_3: f32 = 155.56;
pub const E3: f32 = 164.81;
pub const F3: f32 = 174.61;
pub const F_SH_3: f32 = 185.0;
pub const G3: f32 = 196.0;
pub const G_SH_3: f32 = 207.65;
pub const A3: f32 = 220.0;
pub const A_SH_3: f32 = 233.08;
pub const B3: f32 = 246.94;
pub const C4: f32 = 261.63;
pub const C_SH_4: f32 = 277.18;
pub const D4: f32 = 293.66;
pub const D_SH_4: f32 = 311.13;
pub const E4: f32 = 329.63;
pub const F4: f32 = 349.23;
pub const F_SH_4: f32 = 369.99;
pub const G4: f32 = 392.0;
pub const G_SH_4: f32 = 415.3;
pub const A4: f32 = 440.0;
pub const A_SH_4: f32 = 466.16;
pub const B4: f32 = 493.88;
pub const C5: f32 = 523.25;
pub const C_SH_5: f32 = 554.37;
pub const D5: f32 = 587.33;
pub const D_SH_5: f32 = 622.25;
pub const E5: f32 = 659.25;
pub const F5: f32 = 698.46;
pub const F_SH_5: f32 = 739.99;
pub const G5: f32 = 783.99;
pub const G_SH_5: f32 = 830.61;
pub const A5: f32 = 880.0;
pub const A_SH_5: f32 = 932.33;
pub const B5: f32 = 987.77;
pub const C6: f32 = 1046.5;
pub const C_SH_6: f32 = 1108.73;
pub const D6: f32 = 1174.66;
pub const D_SH_6: f32 = 1244.51;
pub const E6: f32 = 1318.51;
pub const F6: f32 = 1396.91;
pub const F_SH_6: f32 = 1479.98;
pub const G6: f32 = 1567.98;
pub const G_SH_6: f32 = 1661.22;
pub const A6: f32 = 1760.0;
pub const A_SH_6: f32 = 1864.66;
pub const B6: f32 = 1975.53;
pub const C7: f32 = 2093.0;
pub const C_SH_7: f32 = 2217.46;
pub const D7: f32 = 2349.32;
pub const D_SH_7: f32 = 2489.02;
pub const E7: f32 = 2637.02;
pub const F7: f32 = 2793.83;
pub const F_SH_7: f32 = 2959.96;
pub const G7: f32 = 3135.96;
pub const G_SH_7: f32 = 3322.44;
pub const A7: f32 = 3520.0;
pub const A_SH_7: f32 = 3729.31;
pub const B7: f32 = 3951.07;
pub const C8: f32 = 4186.01;
pub const C_SH_8: f32 = 4434.92;
pub const D8: f32 = 4698.63;
pub const D_SH_8: f32 = 4978.03;
pub const E8: f32 = 5274.04;
pub const F8: f32 = 5587.65;
pub const F_SH_8: f32 = 5919.91;
pub const G8: f32 = 6271.93;
pub const G_SH_8: f32 = 6644.88;
pub const A8: f32 = 7040.0;
pub const A_SH_8: f32 = 7458.62;
pub const B8: f32 = 7902.13;

/* Scales */
pub static C0_MINOR_SCALE: Scale = [C0, D0, D_SH_0, F0, G0, G_SH_0, A_SH_0];
pub static C_SH_0_MINOR_SCALE: Scale = [C_SH_0, D_SH_0, E0, F_SH_0, G_SH_0, A0, B0];
pub static D0_MINOR_SCALE: Scale = [D0, E0, F0, G0, A0, A_SH_0, C1];
pub static D_SH_0_MINOR_SCALE: Scale = [D_SH_0, F0, F_SH_0, G_SH_0, A_SH_0, B0, C_SH_1];
pub static E0_MINOR_SCALE: Scale = [E0, F_SH_0, G0, A0, B0, C1, D1];
pub static F0_MINOR_SCALE: Scale = [F0, G0, G_SH_0, A_SH_0, C1, C_SH_1, D_SH_1];
pub static F_SH_0_MINOR_SCALE: Scale = [F_SH_0, G_SH_0, A0, B0, C_SH_1, D1, E1];
pub static G0_MINOR_SCALE: Scale = [G0, A0, A_SH_0, C1, D1, D_SH_1, F1];
pub static G_SH_0_MINOR_SCALE: Scale = [G_SH_0, A_SH_0, B0, C_SH_1, D_SH_1, E1, F_SH_1];
pub static A0_MINOR_SCALE: Scale = [A0, B0, C1, D1, E1, F1, G1];
pub static A_SH_0_MINOR_SCALE: Scale = [A_SH_0, C1, C_SH_1, D_SH_1, F1, F_SH_1, G_SH_1];
pub static B0_MINOR_SCALE: Scale = [B0, C_SH_1, D1, E1, F_SH_1, G1, A1];
pub static C1_MINOR_SCALE: Scale = [C1, D1, D_SH_1, F1, G1, G_SH_1, A_SH_1];
pub static C_SH_1_MINOR_SCALE: Scale = [C_SH_1, D_SH_1, E1, F_SH_1, G_SH_1, A1, B1];
pub static D1_MINOR_SCALE: Scale = [D1, E1, F1, G1, A1, A_SH_1, C2];
pub static D_SH_1_MINOR_SCALE: Scale = [D_SH_1, F1, F_SH_1, G_SH_1, A_SH_1, B1, C_SH_2];
pub static E1_MINOR_SCALE: Scale = [E1, F_SH_1, G1, A1, B1, C2, D2];
pub static F1_MINOR_SCALE: Scale = [F1, G1, G_SH_1, A_SH_1, C2, C_SH_2, D_SH_2];
pub static F_SH_1_MINOR_SCALE: Scale = [F_SH_1, G_SH_1, A1, B1, C_SH_2, D2, E2];
pub static G1_MINOR_SCALE: Scale = [G1, A1, A_SH_1, C2, D2, D_SH_2, F2];
pub static G_SH_1_MINOR_SCALE: Scale = [G_SH_1, A_SH_1, B1, C_SH_2, D_SH_2, E2, F_SH_2];
pub static A1_MINOR_SCALE: Scale = [A1, B1, C2, D2, E2, F2, G2];
pub static A_SH_1_MINOR_SCALE: Scale = [A_SH_1, C2, C_SH_2, D_SH_2, F2, F_SH_2, G_SH_2];
pub static B1_MINOR_SCALE: Scale = [B1, C_SH_2, D2, E2, F_SH_2, G2, A2];
pub static C2_MINOR_SCALE: Scale = [C2, D2, D_SH_2, F2, G2, G_SH_2, A_SH_2];
pub static C_SH_2_MINOR_SCALE: Scale = [C_SH_2, D_SH_2, E2, F_SH_2, G_SH_2, A2, B2];
pub static D2_MINOR_SCALE: Scale = [D2, E2, F2, G2, A2, A_SH_2, C3];
pub static D_SH_2_MINOR_SCALE: Scale = [D_SH_2, F2, F_SH_2, G_SH_2, A_SH_2, B2, C_SH_3];
pub static E2_MINOR_SCALE: Scale = [E2, F_SH_2, G2, A2, B2, C3, D3];
pub static F2_MINOR_SCALE: Scale = [F2, G2, G_SH_2, A_SH_2, C3, C_SH_3, D_SH_3];
pub static F_SH_2_MINOR_SCALE: Scale = [F_SH_2, G_SH_2, A2, B2, C_SH_3, D3, E3];
pub static G2_MINOR_SCALE: Scale = [G2, A2, A_SH_2, C3, D3, D_SH_3, F3];
pub static G_SH_2_MINOR_SCALE: Scale = [G_SH_2, A_SH_2, B2, C_SH_3, D_SH_3, E3, F_SH_3];
pub static A2_MINOR_SCALE: Scale = [A2, B2, C3, D3, E3, F3, G3];
pub static A_SH_2_MINOR_SCALE: Scale = [A_SH_2, C3, C_SH_3, D_SH_3, F3, F_SH_3, G_SH_3];
pub static B2_MINOR_SCALE: Scale = [B2, C_SH_3, D3, E3, F_SH_3, G3, A3];
pub static C3_MINOR_SCALE: Scale = [C3, D3, D_SH_3, F3, G3, G_SH_3, A_SH_3];
pub static C_SH_3_MINOR_SCALE: Scale = [C_SH_3, D_SH_3, E3, F_SH_3, G_SH_3, A3, B3];
pub static D3_MINOR_SCALE: Scale = [D3, E3, F3, G3, A3, A_SH_3, C4];
pub static D_SH_3_MINOR_SCALE: Scale = [D_SH_3, F3, F_SH_3, G_SH_3, A_SH_3, B3, C_SH_4];
pub static E3_MINOR_SCALE: Scale = [E3, F_SH_3, G3, A3, B3, C4, D4];
pub static F3_MINOR_SCALE: Scale = [F3, G3, G_SH_3, A_SH_3, C4, C_SH_4, D_SH_4];
pub static F_SH_3_MINOR_SCALE: Scale = [F_SH_3, G_SH_3, A3, B3, C_SH_4, D4, E4];
pub static G3_MINOR_SCALE: Scale = [G3, A3, A_SH_3, C4, D4, D_SH_4, F4];
pub static G_SH_3_MINOR_SCALE: Scale = [G_SH_3, A_SH_3, B3, C_SH_4, D_SH_4, E4, F_SH_4];
pub static A3_MINOR_SCALE: Scale = [A3, B3, C4, D4, E4, F4, G4];
pub static A_SH_3_MINOR_SCALE: Scale = [A_SH_3, C4, C_SH_4, D_SH_4, F4, F_SH_4, G_SH_4];
pub static B3_MINOR_SCALE: Scale = [B3, C_SH_4, D4, E4, F_SH_4, G4, A4];
pub static C4_MINOR_SCALE: Scale = [C4, D4, D_SH_4, F4, G4, G_SH_4, A_SH_4];
pub static C_SH_4_MINOR_SCALE: Scale = [C_SH_4, D_SH_4, E4, F_SH_4, G_SH_4, A4, B4];
pub static D4_MINOR_SCALE: Scale = [D4, E4, F4, G4, A4, A_SH_4, C5];
pub static D_SH_4_MINOR_SCALE: Scale = [D_SH_4, F4, F_SH_4, G_SH_4, A_SH_4, B4, C_SH_5];
pub static E4_MINOR_SCALE: Scale = [E4, F_SH_4, G4, A4, B4, C5, D5];
pub static F4_MINOR_SCALE: Scale = [F4, G4, G_SH_4, A_SH_4, C5, C_SH_5, D_SH_5];
pub static F_SH_4_MINOR_SCALE: Scale = [F_SH_4, G_SH_4, A4, B4, C_SH_5, D5, E5];
pub static G4_MINOR_SCALE: Scale = [G4, A4, A_SH_4, C5, D5, D_SH_5, F5];
pub static G_SH_4_MINOR_SCALE: Scale = [G_SH_4, A_SH_4, B4, C_SH_5, D_SH_5, E5, F_SH_5];
pub static A4_MINOR_SCALE: Scale = [A4, B4, C5, D5, E5, F5, G5];
pub static A_SH_4_MINOR_SCALE: Scale = [A_SH_4, C5, C_SH_5, D_SH_5, F5, F_SH_5, G_SH_5];
pub static B4_MINOR_SCALE: Scale = [B4, C_SH_5, D5, E5, F_SH_5, G5, A5];
pub static C5_MINOR_SCALE: Scale = [C5, D5, D_SH_5, F5, G5, G_SH_5, A_SH_5];
pub static C_SH_5_MINOR_SCALE: Scale = [C_SH_5, D_SH_5, E5, F_SH_5, G_SH_5, A5, B5];
pub static D5_MINOR_SCALE: Scale = [D5, E5, F5, G5, A5, A_SH_5, C6];
pub static D_SH_5_MINOR_SCALE: Scale = [D_SH_5, F5, F_SH_5, G_SH_5, A_SH_5, B5, C_SH_6];
pub static E5_MINOR_SCALE: Scale = [E5, F_SH_5, G5, A5, B5, C6, D6];
pub static F5_MINOR_SCALE: Scale = [F5, G5, G_SH_5, A_SH_5, C6, C_SH_6, D_SH_6];
pub static F_SH_5_MINOR_SCALE: Scale = [F_SH_5, G_SH_5, A5, B5, C_SH_6, D6, E6];
pub static G5_MINOR_SCALE: Scale = [G5, A5, A_SH_5, C6, D6, D_SH_6, F6];
pub static G_SH_5_MINOR_SCALE: Scale = [G_SH_5, A_SH_5, B5, C_SH_6, D_SH_6, E6, F_SH_6];
pub static A5_MINOR_SCALE: Scale = [A5, B5, C6, D6, E6, F6, G6];
pub static A_SH_5_MINOR_SCALE: Scale = [A_SH_5, C6, C_SH_6, D_SH_6, F6, F_SH_6, G_SH_6];
pub static B5_MINOR_SCALE: Scale = [B5, C_SH_6, D6, E6, F_SH_6, G6, A6];
pub static C6_MINOR_SCALE: Scale = [C6, D6, D_SH_6, F6, G6, G_SH_6, A_SH_6];
pub static C_SH_6_MINOR_SCALE: Scale = [C_SH_6, D_SH_6, E6, F_SH_6, G_SH_6, A6, B6];
pub static D6_MINOR_SCALE: Scale = [D6, E6, F6, G6, A6, A_SH_6, C7];
pub static D_SH_6_MINOR_SCALE: Scale = [D_SH_6, F6, F_SH_6, G_SH_6, A_SH_6, B6, C_SH_7];
pub static E6_MINOR_SCALE: Scale = [E6, F_SH_6, G6, A6, B6, C7, D7];
pub static F6_MINOR_SCALE: Scale = [F6, G6, G_SH_6, A_SH_6, C7, C_SH_7, D_SH_7];
pub static F_SH_6_MINOR_SCALE: Scale = [F_SH_6, G_SH_6, A6, B6, C_SH_7, D7, E7];
pub static G6_MINOR_SCALE: Scale = [G6, A6, A_SH_6, C7, D7, D_SH_7, F7];
pub static G_SH_6_MINOR_SCALE: Scale = [G_SH_6, A_SH_6, B6, C_SH_7, D_SH_7, E7, F_SH_7];
pub static A6_MINOR_SCALE: Scale = [A6, B6, C7, D7, E7, F7, G7];
pub static A_SH_6_MINOR_SCALE: Scale = [A_SH_6, C7, C_SH_7, D_SH_7, F7, F_SH_7, G_SH_7];
pub static B6_MINOR_SCALE: Scale = [B6, C_SH_7, D7, E7, F_SH_7, G7, A7];

pub static C0_MAJOR_SCALE: Scale = [C0, D0, E0, F0, G0, A0, B0];
pub static C_SH_0_MAJOR_SCALE: Scale = [C_SH_0, D_SH_0, F0, F_SH_0, G_SH_0, A_SH_0, C1];
pub static D0_MAJOR_SCALE: Scale = [D0, E0, F_SH_0, G0, A0, B0, C_SH_1];
pub static D_SH_0_MAJOR_SCALE: Scale = [D_SH_0, F0, G0, G_SH_0, A_SH_0, C1, D1];
pub static E0_MAJOR_SCALE: Scale = [E0, F_SH_0, G_SH_0, A0, B0, C_SH_1, D_SH_1];
pub static F0_MAJOR_SCALE: Scale = [F0, G0, A0, A_SH_0, C1, D1, E1];
pub static F_SH_0_MAJOR_SCALE: Scale = [F_SH_0, G_SH_0, A_SH_0, B0, C_SH_1, D_SH_1, F1];
pub static G0_MAJOR_SCALE: Scale = [G0, A0, B0, C1, D1, E1, F_SH_1];
pub static G_SH_0_MAJOR_SCALE: Scale = [G_SH_0, A_SH_0, C1, C_SH_1, D_SH_1, F1, G1];
pub static A0_MAJOR_SCALE: Scale = [A0, B0, C_SH_1, D1, E1, F_SH_1, G_SH_1];
pub static A_SH_0_MAJOR_SCALE: Scale = [A_SH_0, C1, D1, D_SH_1, F1, G1, A1];
pub static B0_MAJOR_SCALE: Scale = [B0, C_SH_1, D_SH_1, E1, F_SH_1, G_SH_1, A_SH_1];
pub static C1_MAJOR_SCALE: Scale = [C1, D1, E1, F1, G1, A1, B1];
pub static C_SH_1_MAJOR_SCALE: Scale = [C_SH_1, D_SH_1, F1, F_SH_1, G_SH_1, A_SH_1, C2];
pub static D1_MAJOR_SCALE: Scale = [D1, E1, F_SH_1, G1, A1, B1, C_SH_2];
pub static D_SH_1_MAJOR_SCALE: Scale = [D_SH_1, F1, G1, G_SH_1, A_SH_1, C2, D2];
pub static E1_MAJOR_SCALE: Scale = [E1, F_SH_1, G_SH_1, A1, B1, C_SH_2, D_SH_2];
pub static F1_MAJOR_SCALE: Scale = [F1, G1, A1, A_SH_1, C2, D2, E2];
pub static F_SH_1_MAJOR_SCALE: Scale = [F_SH_1, G_SH_1, A_SH_1, B1, C_SH_2, D_SH_2, F2];
pub static G1_MAJOR_SCALE: Scale = [G1, A1, B1, C2, D2, E2, F_SH_2];
pub static G_SH_1_MAJOR_SCALE: Scale = [G_SH_1, A_SH_1, C2, C_SH_2, D_SH_2, F2, G2];
pub static A1_MAJOR_SCALE: Scale = [A1, B1, C_SH_2, D2, E2, F_SH_2, G_SH_2];
pub static A_SH_1_MAJOR_SCALE: Scale = [A_SH_1, C2, D2, D_SH_2, F2, G2, A2];
pub static B1_MAJOR_SCALE: Scale = [B1, C_SH_2, D_SH_2, E2, F_SH_2, G_SH_2, A_SH_2];
pub static C2_MAJOR_SCALE: Scale = [C2, D2, E2, F2, G2, A2, B2];
pub static C_SH_2_MAJOR_SCALE: Scale = [C_SH_2, D_SH_2, F2, F_SH_2, G_SH_2, A_SH_2, C3];
pub static D2_MAJOR_SCALE: Scale = [D2, E2, F_SH_2, G2, A2, B2, C_SH_3];
pub static D_SH_2_MAJOR_SCALE: Scale = [D_SH_2, F2, G2, G_SH_2, A_SH_2, C3, D3];
pub static E2_MAJOR_SCALE: Scale = [E2, F_SH_2, G_SH_2, A2, B2, C_SH_3, D_SH_3];
pub static F2_MAJOR_SCALE: Scale = [F2, G2, A2, A_SH_2, C3, D3, E3];
pub static F_SH_2_MAJOR_SCALE: Scale = [F_SH_2, G_SH_2, A_SH_2, B2, C_SH_3, D_SH_3, F3];
pub static G2_MAJOR_SCALE: Scale = [G2, A2, B2, C3, D3, E3, F_SH_3];
pub static G_SH_2_MAJOR_SCALE: Scale = [G_SH_2, A_SH_2, C3, C_SH_3, D_SH_3, F3, G3];
pub static A2_MAJOR_SCALE: Scale = [A2, B2, C_SH_3, D3, E3, F_SH_3, G_SH_3];
pub static A_SH_2_MAJOR_SCALE: Scale = [A_SH_2, C3, D3, D_SH_3, F3, G3, A3];
pub static B2_MAJOR_SCALE: Scale = [B2, C_SH_3, D_SH_3, E3, F_SH_3, G_SH_3, A_SH_3];
pub static C3_MAJOR_SCALE: Scale = [C3, D3, E3, F3, G3, A3, B3];
pub static C_SH_3_MAJOR_SCALE: Scale = [C_SH_3, D_SH_3, F3, F_SH_3, G_SH_3, A_SH_3, C4];
pub static D3_MAJOR_SCALE: Scale = [D3, E3, F_SH_3, G3, A3, B3, C_SH_4];
pub static D_SH_3_MAJOR_SCALE: Scale = [D_SH_3, F3, G3, G_SH_3, A_SH_3, C4, D4];
pub static E3_MAJOR_SCALE: Scale = [E3, F_SH_3, G_SH_3, A3, B3, C_SH_4, D_SH_4];
pub static F3_MAJOR_SCALE: Scale = [F3, G3, A3, A_SH_3, C4, D4, E4];
pub static F_SH_3_MAJOR_SCALE: Scale = [F_SH_3, G_SH_3, A_SH_3, B3, C_SH_4, D_SH_4, F4];
pub static G3_MAJOR_SCALE: Scale = [G3, A3, B3, C4, D4, E4, F_SH_4];
pub static G_SH_3_MAJOR_SCALE: Scale = [G_SH_3, A_SH_3, C4, C_SH_4, D_SH_4, F4, G4];
pub static A3_MAJOR_SCALE: Scale = [A3, B3, C_SH_4, D4, E4, F_SH_4, G_SH_4];
pub static A_SH_3_MAJOR_SCALE: Scale = [A_SH_3, C4, D4, D_SH_4, F4, G4, A4];
pub static B3_MAJOR_SCALE: Scale = [B3, C_SH_4, D_SH_4, E4, F_SH_4, G_SH_4, A_SH_4];
pub static C4_MAJOR_SCALE: Scale = [C4, D4, E4, F4, G4, A4, B4];
pub static C_SH_4_MAJOR_SCALE: Scale = [C_SH_4, D_SH_4, F4, F_SH_4, G_SH_4, A_SH_4, C5];
pub static D4_MAJOR_SCALE: Scale = [D4, E4, F_SH_4, G4, A4, B4, C_SH_5];
pub static D_SH_4_MAJOR_SCALE: Scale = [D_SH_4, F4, G4, G_SH_4, A_SH_4, C5, D5];
pub static E4_MAJOR_SCALE: Scale = [E4, F_SH_4, G_SH_4, A4, B4, C_SH_5, D_SH_5];
pub static F4_MAJOR_SCALE: Scale = [F4, G4, A4, A_SH_4, C5, D5, E5];
pub static F_SH_4_MAJOR_SCALE: Scale = [F_SH_4, G_SH_4, A_SH_4, B4, C_SH_5, D_SH_5, F5];
pub static G4_MAJOR_SCALE: Scale = [G4, A4, B4, C5, D5, E5, F_SH_5];
pub static G_SH_4_MAJOR_SCALE: Scale = [G_SH_4, A_SH_4, C5, C_SH_5, D_SH_5, F5, G5];
pub static A4_MAJOR_SCALE: Scale = [A4, B4, C_SH_5, D5, E5, F_SH_5, G_SH_5];
pub static A_SH_4_MAJOR_SCALE: Scale = [A_SH_4, C5, D5, D_SH_5, F5, G5, A5];
pub static B4_MAJOR_SCALE: Scale = [B4, C_SH_5, D_SH_5, E5, F_SH_5, G_SH_5, A_SH_5];
pub static C5_MAJOR_SCALE: Scale = [C5, D5, E5, F5, G5, A5, B5];
pub static C_SH_5_MAJOR_SCALE: Scale = [C_SH_5, D_SH_5, F5, F_SH_5, G_SH_5, A_SH_5, C6];
pub static D5_MAJOR_SCALE: Scale = [D5, E5, F_SH_5, G5, A5, B5, C_SH_6];
pub static D_SH_5_MAJOR_SCALE: Scale = [D_SH_5, F5, G5, G_SH_5, A_SH_5, C6, D6];
pub static E5_MAJOR_SCALE: Scale = [E5, F_SH_5, G_SH_5, A5, B5, C_SH_6, D_SH_6];
pub static F5_MAJOR_SCALE: Scale = [F5, G5, A5, A_SH_5, C6, D6, E6];
pub static F_SH_5_MAJOR_SCALE: Scale = [F_SH_5, G_SH_5, A_SH_5, B5, C_SH_6, D_SH_6, F6];
pub static G5_MAJOR_SCALE: Scale = [G5, A5, B5, C6, D6, E6, F_SH_6];
pub static G_SH_5_MAJOR_SCALE: Scale = [G_SH_5, A_SH_5, C6, C_SH_6, D_SH_6, F6, G6];
pub static A5_MAJOR_SCALE: Scale = [A5, B5, C_SH_6, D6, E6, F_SH_6, G_SH_6];
pub static A_SH_5_MAJOR_SCALE: Scale = [A_SH_5, C6, D6, D_SH_6, F6, G6, A6];
pub static B5_MAJOR_SCALE: Scale = [B5, C_SH_6, D_SH_6, E6, F_SH_6, G_SH_6, A_SH_6];
pub static C6_MAJOR_SCALE: Scale = [C6, D6, E6, F6, G6, A6, B6];
pub static C_SH_6_MAJOR_SCALE: Scale = [C_SH_6, D_SH_6, F6, F_SH_6, G_SH_6, A_SH_6, C7];
pub static D6_MAJOR_SCALE: Scale = [D6, E6, F_SH_6, G6, A6, B6, C_SH_7];
pub static D_SH_6_MAJOR_SCALE: Scale = [D_SH_6, F6, G6, G_SH_6, A_SH_6, C7, D7];
pub static E6_MAJOR_SCALE: Scale = [E6, F_SH_6, G_SH_6, A6, B6, C_SH_7, D_SH_7];
pub static F6_MAJOR_SCALE: Scale = [F6, G6, A6, A_SH_6, C7, D7, E7];
pub static F_SH_6_MAJOR_SCALE: Scale = [F_SH_6, G_SH_6, A_SH_6, B6, C_SH_7, D_SH_7, F7];
pub static G6_MAJOR_SCALE: Scale = [G6, A6, B6, C7, D7, E7, F_SH_7];
pub static G_SH_6_MAJOR_SCALE: Scale = [G_SH_6, A_SH_6, C7, C_SH_7, D_SH_7, F7, G7];
pub static A6_MAJOR_SCALE: Scale = [A6, B6, C_SH_7, D7, E7, F_SH_7, G_SH_7];
pub static A_SH_6_MAJOR_SCALE: Scale = [A_SH_6, C7, D7, D_SH_7, F7, G7, A7];
pub static B6_MAJOR_SCALE: Scale = [B6, C_SH_7, D_SH_7, E7, F_SH_7, G_SH_7, A_SH_7];

/* Leads */
pub static AM_LEAD: Lead = [&A4_MINOR_SCALE, &A5_MINOR_SCALE, &A6_MINOR_SCALE];
pub static A_SH_M_LEAD: Lead = [
  &A_SH_4_MINOR_SCALE,
  &A_SH_5_MINOR_SCALE,
  &A_SH_6_MINOR_SCALE,
];
pub static BM_LEAD: Lead = [&B4_MINOR_SCALE, &B5_MINOR_SCALE, &B6_MINOR_SCALE];
pub static CM_LEAD: Lead = [&C4_MINOR_SCALE, &C5_MINOR_SCALE, &C6_MINOR_SCALE];
pub static C_SH_M_LEAD: Lead = [
  &C_SH_4_MINOR_SCALE,
  &C_SH_5_MINOR_SCALE,
  &C_SH_6_MINOR_SCALE,
];
pub static DM_LEAD: Lead = [&D4_MINOR_SCALE, &D5_MINOR_SCALE, &D6_MINOR_SCALE];
pub static D_SH_M_LEAD: Lead = [
  &D_SH_4_MINOR_SCALE,
  &D_SH_5_MINOR_SCALE,
  &D_SH_6_MINOR_SCALE,
];
pub static EM_LEAD: Lead = [&E4_MINOR_SCALE, &E5_MINOR_SCALE, &E6_MINOR_SCALE];
pub static FM_LEAD: Lead = [&F4_MINOR_SCALE, &F5_MINOR_SCALE, &F6_MINOR_SCALE];
pub static F_SH_M_LEAD: Lead = [
  &F_SH_4_MINOR_SCALE,
  &F_SH_5_MINOR_SCALE,
  &F_SH_6_MINOR_SCALE,
];
pub static GM_LEAD: Lead = [&G4_MINOR_SCALE, &G5_MINOR_SCALE, &G6_MINOR_SCALE];
pub static G_SH_M_LEAD: Lead = [
  &G_SH_4_MINOR_SCALE,
  &G_SH_5_MINOR_SCALE,
  &G_SH_6_MINOR_SCALE,
];

pub static A_LEAD: Lead = [&A4_MAJOR_SCALE, &A5_MAJOR_SCALE, &A6_MAJOR_SCALE];
pub static A_SH_LEAD: Lead = [
  &A_SH_4_MAJOR_SCALE,
  &A_SH_5_MAJOR_SCALE,
  &A_SH_6_MAJOR_SCALE,
];
pub static B_LEAD: Lead = [&B4_MAJOR_SCALE, &B5_MAJOR_SCALE, &B6_MAJOR_SCALE];
pub static C_LEAD: Lead = [&C4_MAJOR_SCALE, &C5_MAJOR_SCALE, &C6_MAJOR_SCALE];
pub static C_SH_LEAD: Lead = [
  &C_SH_4_MAJOR_SCALE,
  &C_SH_5_MAJOR_SCALE,
  &C_SH_6_MAJOR_SCALE,
];
pub static D_LEAD: Lead = [&D4_MAJOR_SCALE, &D5_MAJOR_SCALE, &D6_MAJOR_SCALE];
pub static D_SH_LEAD: Lead = [
  &D_SH_4_MAJOR_SCALE,
  &D_SH_5_MAJOR_SCALE,
  &D_SH_6_MAJOR_SCALE,
];
pub static E_LEAD: Lead = [&E4_MAJOR_SCALE, &E5_MAJOR_SCALE, &E6_MAJOR_SCALE];
pub static F_LEAD: Lead = [&F4_MAJOR_SCALE, &F5_MAJOR_SCALE, &F6_MAJOR_SCALE];
pub static F_SH_LEAD: Lead = [
  &F_SH_4_MAJOR_SCALE,
  &F_SH_5_MAJOR_SCALE,
  &F_SH_6_MAJOR_SCALE,
];
pub static G_LEAD: Lead = [&G4_MAJOR_SCALE, &G5_MAJOR_SCALE, &G6_MAJOR_SCALE];
pub static G_SH_LEAD: Lead = [
  &G_SH_4_MAJOR_SCALE,
  &G_SH_5_MAJOR_SCALE,
  &G_SH_6_MAJOR_SCALE,
];

/* Chords */
pub static C2_MINOR_CHORD: Chord = [C2, D_SH_2, F2];
pub static C_SH_2_MINOR_CHORD: Chord = [C_SH_2, E2, F_SH_2];
pub static D2_MINOR_CHORD: Chord = [D2, F2, G2];
pub static D_SH_2_MINOR_CHORD: Chord = [D_SH_2, F_SH_2, G_SH_2];
pub static E2_MINOR_CHORD: Chord = [E2, G2, A2];
pub static F2_MINOR_CHORD: Chord = [F2, G_SH_2, A_SH_2];
pub static F_SH_2_MINOR_CHORD: Chord = [F_SH_2, A2, B2];
pub static G2_MINOR_CHORD: Chord = [G2, A_SH_2, C3];
pub static G_SH_2_MINOR_CHORD: Chord = [G_SH_2, B2, C_SH_3];
pub static A2_MINOR_CHORD: Chord = [A2, C3, D3];
pub static A_SH_2_MINOR_CHORD: Chord = [A_SH_2, C_SH_3, D_SH_3];
pub static B2_MINOR_CHORD: Chord = [B2, D3, E3];
pub static C3_MINOR_CHORD: Chord = [C3, D_SH_3, F3];
pub static C_SH_3_MINOR_CHORD: Chord = [C_SH_3, E3, F_SH_3];
pub static D3_MINOR_CHORD: Chord = [D3, F3, G3];
pub static D_SH_3_MINOR_CHORD: Chord = [D_SH_3, F_SH_3, G_SH_3];
pub static E3_MINOR_CHORD: Chord = [E3, G3, A3];
pub static F3_MINOR_CHORD: Chord = [F3, G_SH_3, A_SH_3];
pub static F_SH_3_MINOR_CHORD: Chord = [F_SH_3, A3, B3];
pub static G3_MINOR_CHORD: Chord = [G3, A_SH_3, C4];
pub static G_SH_3_MINOR_CHORD: Chord = [G_SH_3, B3, C_SH_4];
pub static A3_MINOR_CHORD: Chord = [A3, C4, D4];
pub static A_SH_3_MINOR_CHORD: Chord = [A_SH_3, C_SH_4, D_SH_4];
pub static B3_MINOR_CHORD: Chord = [B3, D4, E4];
pub static C4_MINOR_CHORD: Chord = [C4, D_SH_4, F4];
pub static C_SH_4_MINOR_CHORD: Chord = [C_SH_4, E4, F_SH_4];
pub static D4_MINOR_CHORD: Chord = [D4, F4, G4];
pub static D_SH_4_MINOR_CHORD: Chord = [D_SH_4, F_SH_4, G_SH_4];
pub static E4_MINOR_CHORD: Chord = [E4, G4, A4];
pub static F4_MINOR_CHORD: Chord = [F4, G_SH_4, A_SH_4];
pub static F_SH_4_MINOR_CHORD: Chord = [F_SH_4, A4, B4];
pub static G4_MINOR_CHORD: Chord = [G4, A_SH_4, C5];
pub static G_SH_4_MINOR_CHORD: Chord = [G_SH_4, B4, C_SH_5];
pub static A4_MINOR_CHORD: Chord = [A4, C5, D5];
pub static A_SH_4_MINOR_CHORD: Chord = [A_SH_4, C_SH_5, D_SH_5];
pub static B4_MINOR_CHORD: Chord = [B4, D5, E5];
pub static C5_MINOR_CHORD: Chord = [C5, D_SH_5, F5];
pub static C_SH_5_MINOR_CHORD: Chord = [C_SH_5, E5, F_SH_5];
pub static D5_MINOR_CHORD: Chord = [D5, F5, G5];
pub static D_SH_5_MINOR_CHORD: Chord = [D_SH_5, F_SH_5, G_SH_5];
pub static E5_MINOR_CHORD: Chord = [E5, G5, A5];
pub static F5_MINOR_CHORD: Chord = [F5, G_SH_5, A_SH_5];
pub static F_SH_5_MINOR_CHORD: Chord = [F_SH_5, A5, B5];
pub static G5_MINOR_CHORD: Chord = [G5, A_SH_5, C6];
pub static G_SH_5_MINOR_CHORD: Chord = [G_SH_5, B5, C_SH_6];
pub static A5_MINOR_CHORD: Chord = [A5, C6, D6];
pub static A_SH_5_MINOR_CHORD: Chord = [A_SH_5, C_SH_6, D_SH_6];
pub static B5_MINOR_CHORD: Chord = [B5, D6, E6];

pub static C2_MAJOR_CHORD: Chord = [C2, E2, F2];
pub static C_SH_2_MAJOR_CHORD: Chord = [C_SH_2, F2, F_SH_2];
pub static D2_MAJOR_CHORD: Chord = [D2, F_SH_2, G2];
pub static D_SH_2_MAJOR_CHORD: Chord = [D_SH_2, G2, G_SH_2];
pub static E2_MAJOR_CHORD: Chord = [E2, G_SH_2, A2];
pub static F2_MAJOR_CHORD: Chord = [F2, A2, A_SH_2];
pub static F_SH_2_MAJOR_CHORD: Chord = [F_SH_2, A_SH_2, B2];
pub static G2_MAJOR_CHORD: Chord = [G2, B2, C3];
pub static G_SH_2_MAJOR_CHORD: Chord = [G_SH_2, C3, C_SH_3];
pub static A2_MAJOR_CHORD: Chord = [A2, C_SH_3, D3];
pub static A_SH_2_MAJOR_CHORD: Chord = [A_SH_2, D3, D_SH_3];
pub static B2_MAJOR_CHORD: Chord = [B2, D_SH_3, E3];
pub static C3_MAJOR_CHORD: Chord = [C3, E3, F3];
pub static C_SH_3_MAJOR_CHORD: Chord = [C_SH_3, F3, F_SH_3];
pub static D3_MAJOR_CHORD: Chord = [D3, F_SH_3, G3];
pub static D_SH_3_MAJOR_CHORD: Chord = [D_SH_3, G3, G_SH_3];
pub static E3_MAJOR_CHORD: Chord = [E3, G_SH_3, A3];
pub static F3_MAJOR_CHORD: Chord = [F3, A3, A_SH_3];
pub static F_SH_3_MAJOR_CHORD: Chord = [F_SH_3, A_SH_3, B3];
pub static G3_MAJOR_CHORD: Chord = [G3, B3, C4];
pub static G_SH_3_MAJOR_CHORD: Chord = [G_SH_3, C4, C_SH_4];
pub static A3_MAJOR_CHORD: Chord = [A3, C_SH_4, D4];
pub static A_SH_3_MAJOR_CHORD: Chord = [A_SH_3, D4, D_SH_4];
pub static B3_MAJOR_CHORD: Chord = [B3, D_SH_4, E4];
pub static C4_MAJOR_CHORD: Chord = [C4, E4, F4];
pub static C_SH_4_MAJOR_CHORD: Chord = [C_SH_4, F4, F_SH_4];
pub static D4_MAJOR_CHORD: Chord = [D4, F_SH_4, G4];
pub static D_SH_4_MAJOR_CHORD: Chord = [D_SH_4, G4, G_SH_4];
pub static E4_MAJOR_CHORD: Chord = [E4, G_SH_4, A4];
pub static F4_MAJOR_CHORD: Chord = [F4, A4, A_SH_4];
pub static F_SH_4_MAJOR_CHORD: Chord = [F_SH_4, A_SH_4, B4];
pub static G4_MAJOR_CHORD: Chord = [G4, B4, C5];
pub static G_SH_4_MAJOR_CHORD: Chord = [G_SH_4, C5, C_SH_5];
pub static A4_MAJOR_CHORD: Chord = [A4, C_SH_5, D5];
pub static A_SH_4_MAJOR_CHORD: Chord = [A_SH_4, D5, D_SH_5];
pub static B4_MAJOR_CHORD: Chord = [B4, D_SH_5, E5];
pub static C5_MAJOR_CHORD: Chord = [C5, E5, F5];
pub static C_SH_5_MAJOR_CHORD: Chord = [C_SH_5, F5, F_SH_5];
pub static D5_MAJOR_CHORD: Chord = [D5, F_SH_5, G5];
pub static D_SH_5_MAJOR_CHORD: Chord = [D_SH_5, G5, G_SH_5];
pub static E5_MAJOR_CHORD: Chord = [E5, G_SH_5, A5];
pub static F5_MAJOR_CHORD: Chord = [F5, A5, A_SH_5];
pub static F_SH_5_MAJOR_CHORD: Chord = [F_SH_5, A_SH_5, B5];
pub static G5_MAJOR_CHORD: Chord = [G5, B5, C6];
pub static G_SH_5_MAJOR_CHORD: Chord = [G_SH_5, C6, C_SH_6];
pub static A5_MAJOR_CHORD: Chord = [A5, C_SH_6, D6];
pub static A_SH_5_MAJOR_CHORD: Chord = [A_SH_5, D6, D_SH_6];
pub static B5_MAJOR_CHORD: Chord = [B5, D_SH_6, E6];

pub static C2_DEM_CHORD: Chord = [C2, D_SH_2, F_SH_2];
pub static C_SH_2_DEM_CHORD: Chord = [C_SH_2, E2, G2];
pub static D2_DEM_CHORD: Chord = [D2, F2, G_SH_2];
pub static D_SH_2_DEM_CHORD: Chord = [D_SH_2, F_SH_2, A2];
pub static E2_DEM_CHORD: Chord = [E2, G2, A_SH_2];
pub static F2_DEM_CHORD: Chord = [F2, G_SH_2, B2];
pub static F_SH_2_DEM_CHORD: Chord = [F_SH_2, A2, C3];
pub static G2_DEM_CHORD: Chord = [G2, A_SH_2, C_SH_3];
pub static G_SH_2_DEM_CHORD: Chord = [G_SH_2, B2, D3];
pub static A2_DEM_CHORD: Chord = [A2, C3, D_SH_3];
pub static A_SH_2_DEM_CHORD: Chord = [A_SH_2, C_SH_3, E3];
pub static B2_DEM_CHORD: Chord = [B2, D3, F3];
pub static C3_DEM_CHORD: Chord = [C3, D_SH_3, F_SH_3];
pub static C_SH_3_DEM_CHORD: Chord = [C_SH_3, E3, G3];
pub static D3_DEM_CHORD: Chord = [D3, F3, G_SH_3];
pub static D_SH_3_DEM_CHORD: Chord = [D_SH_3, F_SH_3, A3];
pub static E3_DEM_CHORD: Chord = [E3, G3, A_SH_3];
pub static F3_DEM_CHORD: Chord = [F3, G_SH_3, B3];
pub static F_SH_3_DEM_CHORD: Chord = [F_SH_3, A3, C4];
pub static G3_DEM_CHORD: Chord = [G3, A_SH_3, C_SH_4];
pub static G_SH_3_DEM_CHORD: Chord = [G_SH_3, B3, D4];
pub static A3_DEM_CHORD: Chord = [A3, C4, D_SH_4];
pub static A_SH_3_DEM_CHORD: Chord = [A_SH_3, C_SH_4, E4];
pub static B3_DEM_CHORD: Chord = [B3, D4, F4];
pub static C4_DEM_CHORD: Chord = [C4, D_SH_4, F_SH_4];
pub static C_SH_4_DEM_CHORD: Chord = [C_SH_4, E4, G4];
pub static D4_DEM_CHORD: Chord = [D4, F4, G_SH_4];
pub static D_SH_4_DEM_CHORD: Chord = [D_SH_4, F_SH_4, A4];
pub static E4_DEM_CHORD: Chord = [E4, G4, A_SH_4];
pub static F4_DEM_CHORD: Chord = [F4, G_SH_4, B4];
pub static F_SH_4_DEM_CHORD: Chord = [F_SH_4, A4, C5];
pub static G4_DEM_CHORD: Chord = [G4, A_SH_4, C_SH_5];
pub static G_SH_4_DEM_CHORD: Chord = [G_SH_4, B4, D5];
pub static A4_DEM_CHORD: Chord = [A4, C5, D_SH_5];
pub static A_SH_4_DEM_CHORD: Chord = [A_SH_4, C_SH_5, E5];
pub static B4_DEM_CHORD: Chord = [B4, D5, F5];
pub static C5_DEM_CHORD: Chord = [C5, D_SH_5, F_SH_5];
pub static C_SH_5_DEM_CHORD: Chord = [C_SH_5, E5, G5];
pub static D5_DEM_CHORD: Chord = [D5, F5, G_SH_5];
pub static D_SH_5_DEM_CHORD: Chord = [D_SH_5, F_SH_5, A5];
pub static E5_DEM_CHORD: Chord = [E5, G5, A_SH_5];
pub static F5_DEM_CHORD: Chord = [F5, G_SH_5, B5];
pub static F_SH_5_DEM_CHORD: Chord = [F_SH_5, A5, C6];
pub static G5_DEM_CHORD: Chord = [G5, A_SH_5, C_SH_6];
pub static G_SH_5_DEM_CHORD: Chord = [G_SH_5, B5, D6];
pub static A5_DEM_CHORD: Chord = [A5, C6, D_SH_6];
pub static A_SH_5_DEM_CHORD: Chord = [A_SH_5, C_SH_6, E6];
pub static B5_DEM_CHORD: Chord = [B5, D6, F6];

/* Chord pools */
pub static AM_CHORD_POOL: ChordPool = [
  &A3_MINOR_CHORD,
  &D4_MINOR_CHORD,
  &E4_MINOR_CHORD,
  &C4_MAJOR_CHORD,
  &F4_MAJOR_CHORD,
  &G4_MAJOR_CHORD,
  &B3_DEM_CHORD,
];
pub static A_SH_M_CHORD_POOL: ChordPool = [
  &A_SH_3_MINOR_CHORD,
  &D_SH_4_MINOR_CHORD,
  &F4_MINOR_CHORD,
  &C_SH_4_MAJOR_CHORD,
  &F_SH_4_MAJOR_CHORD,
  &G_SH_4_MAJOR_CHORD,
  &C4_DEM_CHORD,
];
pub static BM_CHORD_POOL: ChordPool = [
  &B3_MINOR_CHORD,
  &E4_MINOR_CHORD,
  &F_SH_4_MINOR_CHORD,
  &D4_MAJOR_CHORD,
  &G4_MAJOR_CHORD,
  &A4_MAJOR_CHORD,
  &C_SH_4_DEM_CHORD,
];
pub static CM_CHORD_POOL: ChordPool = [
  &C3_MINOR_CHORD,
  &F3_MINOR_CHORD,
  &G3_MINOR_CHORD,
  &D_SH_3_MAJOR_CHORD,
  &G_SH_3_MAJOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &D3_DEM_CHORD,
];
pub static C_SH_M_CHORD_POOL: ChordPool = [
  &C_SH_3_MINOR_CHORD,
  &F_SH_3_MINOR_CHORD,
  &G_SH_3_MINOR_CHORD,
  &E3_MAJOR_CHORD,
  &A3_MAJOR_CHORD,
  &B3_MAJOR_CHORD,
  &D_SH_3_DEM_CHORD,
];
pub static DM_CHORD_POOL: ChordPool = [
  &D3_MINOR_CHORD,
  &G3_MINOR_CHORD,
  &A3_MINOR_CHORD,
  &F3_MAJOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &C4_MAJOR_CHORD,
  &E3_DEM_CHORD,
];
pub static D_SH_M_CHORD_POOL: ChordPool = [
  &D_SH_3_MINOR_CHORD,
  &G_SH_3_MINOR_CHORD,
  &A_SH_3_MINOR_CHORD,
  &F_SH_3_MAJOR_CHORD,
  &B3_MAJOR_CHORD,
  &C_SH_4_MAJOR_CHORD,
  &F3_DEM_CHORD,
];
pub static EM_CHORD_POOL: ChordPool = [
  &E3_MINOR_CHORD,
  &A3_MINOR_CHORD,
  &B3_MINOR_CHORD,
  &G3_MAJOR_CHORD,
  &C4_MAJOR_CHORD,
  &D4_MAJOR_CHORD,
  &F_SH_3_DEM_CHORD,
];
pub static FM_CHORD_POOL: ChordPool = [
  &F3_MINOR_CHORD,
  &A_SH_3_MINOR_CHORD,
  &C4_MINOR_CHORD,
  &G_SH_3_MAJOR_CHORD,
  &C_SH_4_MAJOR_CHORD,
  &D_SH_4_MAJOR_CHORD,
  &G3_DEM_CHORD,
];
pub static F_SH_M_CHORD_POOL: ChordPool = [
  &F_SH_3_MINOR_CHORD,
  &B3_MINOR_CHORD,
  &C_SH_4_MINOR_CHORD,
  &A3_MAJOR_CHORD,
  &D4_MAJOR_CHORD,
  &E4_MAJOR_CHORD,
  &G_SH_3_DEM_CHORD,
];
pub static GM_CHORD_POOL: ChordPool = [
  &G3_MINOR_CHORD,
  &C4_MINOR_CHORD,
  &D4_MINOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &D_SH_4_MAJOR_CHORD,
  &F4_MAJOR_CHORD,
  &A3_DEM_CHORD,
];
pub static G_SH_M_CHORD_POOL: ChordPool = [
  &G_SH_3_MINOR_CHORD,
  &C_SH_4_MINOR_CHORD,
  &D_SH_4_MINOR_CHORD,
  &B3_MAJOR_CHORD,
  &E4_MAJOR_CHORD,
  &F_SH_4_MAJOR_CHORD,
  &A_SH_3_DEM_CHORD,
];
pub static A_CHORD_POOL: ChordPool = [
  &B3_MINOR_CHORD,
  &C_SH_4_MINOR_CHORD,
  &F_SH_4_MINOR_CHORD,
  &A3_MAJOR_CHORD,
  &D4_MAJOR_CHORD,
  &E4_MAJOR_CHORD,
  &G_SH_4_DEM_CHORD,
];
pub static A_SH_CHORD_POOL: ChordPool = [
  &C4_MINOR_CHORD,
  &D4_MINOR_CHORD,
  &G4_MINOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &D_SH_4_MAJOR_CHORD,
  &F4_MAJOR_CHORD,
  &A4_DEM_CHORD,
];
pub static B_CHORD_POOL: ChordPool = [
  &C_SH_4_MINOR_CHORD,
  &D_SH_4_MINOR_CHORD,
  &G_SH_4_MINOR_CHORD,
  &B3_MAJOR_CHORD,
  &E4_MAJOR_CHORD,
  &F_SH_4_MAJOR_CHORD,
  &A_SH_4_DEM_CHORD,
];
pub static C_CHORD_POOL: ChordPool = [
  &D3_MINOR_CHORD,
  &E3_MINOR_CHORD,
  &A3_MINOR_CHORD,
  &C3_MAJOR_CHORD,
  &F3_MAJOR_CHORD,
  &G3_MAJOR_CHORD,
  &B3_DEM_CHORD,
];
pub static C_SH_CHORD_POOL: ChordPool = [
  &D_SH_3_MINOR_CHORD,
  &F3_MINOR_CHORD,
  &A_SH_3_MINOR_CHORD,
  &C_SH_3_MAJOR_CHORD,
  &F_SH_3_MAJOR_CHORD,
  &G_SH_3_MAJOR_CHORD,
  &C4_DEM_CHORD,
];
pub static D_CHORD_POOL: ChordPool = [
  &E3_MINOR_CHORD,
  &F_SH_3_MINOR_CHORD,
  &B3_MINOR_CHORD,
  &D3_MAJOR_CHORD,
  &G3_MAJOR_CHORD,
  &A3_MAJOR_CHORD,
  &C_SH_4_DEM_CHORD,
];
pub static D_SH_CHORD_POOL: ChordPool = [
  &F3_MINOR_CHORD,
  &G3_MINOR_CHORD,
  &C4_MINOR_CHORD,
  &D_SH_3_MAJOR_CHORD,
  &G_SH_3_MAJOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &D4_DEM_CHORD,
];
pub static E_CHORD_POOL: ChordPool = [
  &F_SH_3_MINOR_CHORD,
  &G_SH_3_MINOR_CHORD,
  &C_SH_4_MINOR_CHORD,
  &E3_MAJOR_CHORD,
  &A3_MAJOR_CHORD,
  &B3_MAJOR_CHORD,
  &D_SH_4_DEM_CHORD,
];
pub static F_CHORD_POOL: ChordPool = [
  &G3_MINOR_CHORD,
  &A3_MINOR_CHORD,
  &D4_MINOR_CHORD,
  &F3_MAJOR_CHORD,
  &A_SH_3_MAJOR_CHORD,
  &C4_MAJOR_CHORD,
  &E4_DEM_CHORD,
];
pub static F_SH_CHORD_POOL: ChordPool = [
  &G_SH_3_MINOR_CHORD,
  &A_SH_3_MINOR_CHORD,
  &D_SH_4_MINOR_CHORD,
  &F_SH_3_MAJOR_CHORD,
  &B3_MAJOR_CHORD,
  &C_SH_4_MAJOR_CHORD,
  &F4_DEM_CHORD,
];
pub static G_CHORD_POOL: ChordPool = [
  &A3_MINOR_CHORD,
  &B3_MINOR_CHORD,
  &E4_MINOR_CHORD,
  &G3_MAJOR_CHORD,
  &C4_MAJOR_CHORD,
  &D4_MAJOR_CHORD,
  &F_SH_4_DEM_CHORD,
];
pub static G_SH_CHORD_POOL: ChordPool = [
  &A_SH_3_MINOR_CHORD,
  &C4_MINOR_CHORD,
  &F4_MINOR_CHORD,
  &G_SH_3_MAJOR_CHORD,
  &C_SH_4_MAJOR_CHORD,
  &D_SH_4_MAJOR_CHORD,
  &G4_DEM_CHORD,
];

/* Keys */
pub static AM: Key = Key {
  lead: AM_LEAD,
  chords: AM_CHORD_POOL,
};
pub static A: Key = Key {
  lead: A_LEAD,
  chords: A_CHORD_POOL,
};
pub static A_SH_M: Key = Key {
  lead: A_SH_M_LEAD,
  chords: A_SH_M_CHORD_POOL,
};
pub static A_SH: Key = Key {
  lead: A_SH_LEAD,
  chords: A_SH_CHORD_POOL,
};
pub static BM: Key = Key {
  lead: BM_LEAD,
  chords: BM_CHORD_POOL,
};
pub static B: Key = Key {
  lead: B_LEAD,
  chords: B_CHORD_POOL,
};
pub static CM: Key = Key {
  lead: CM_LEAD,
  chords: CM_CHORD_POOL,
};
pub static C: Key = Key {
  lead: C_LEAD,
  chords: C_CHORD_POOL,
};
pub static C_SH_M: Key = Key {
  lead: C_SH_M_LEAD,
  chords: C_SH_M_CHORD_POOL,
};
pub static C_SH: Key = Key {
  lead: C_SH_LEAD,
  chords: C_SH_CHORD_POOL,
};
pub static DM: Key = Key {
  lead: DM_LEAD,
  chords: DM_CHORD_POOL,
};
pub static D: Key = Key {
  lead: D_LEAD,
  chords: D_CHORD_POOL,
};
pub static D_SH_M: Key = Key {
  lead: D_SH_M_LEAD,
  chords: D_SH_M_CHORD_POOL,
};
pub static D_SH: Key = Key {
  lead: D_SH_LEAD,
  chords: D_SH_CHORD_POOL,
};
pub static EM: Key = Key {
  lead: EM_LEAD,
  chords: EM_CHORD_POOL,
};
pub static E: Key = Key {
  lead: E_LEAD,
  chords: E_CHORD_POOL,
};
pub static FM: Key = Key {
  lead: FM_LEAD,
  chords: FM_CHORD_POOL,
};
pub static F: Key = Key {
  lead: F_LEAD,
  chords: F_CHORD_POOL,
};
pub static F_SH_M: Key = Key {
  lead: F_SH_M_LEAD,
  chords: F_SH_M_CHORD_POOL,
};
pub static F_SH: Key = Key {
  lead: F_SH_LEAD,
  chords: F_SH_CHORD_POOL,
};
pub static GM: Key = Key {
  lead: GM_LEAD,
  chords: GM_CHORD_POOL,
};
pub static G: Key = Key {
  lead: G_LEAD,
  chords: G_CHORD_POOL,
};
pub static G_SH_M: Key = Key {
  lead: G_SH_M_LEAD,
  chords: G_SH_M_CHORD_POOL,
};
pub static G_SH: Key = Key {
  lead: G_SH_LEAD,
  chords: G_SH_CHORD_POOL,
};

pub static KEYS: [&Key; 24] = [
  &AM, &A_SH_M, &BM, &CM, &C_SH_M, &DM, &D_SH_M, &EM, &FM, &F_SH_M, &GM, &G_SH_M, &A, &B, &C, &D,
  &E, &F, &G, &A_SH, &C_SH, &D_SH, &F_SH, &G_SH,
];
