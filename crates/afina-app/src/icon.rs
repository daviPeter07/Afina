//! The app icon, drawn in code (no image files to ship): a violet rounded
//! square with a white "equalizer".

use eframe::egui::IconData;

pub fn app_icon() -> IconData {
    const N: usize = 64;
    const RADIUS: f32 = 14.0;
    const BARS: [(f32, f32); 3] = [(14.0, 30.0), (28.0, 14.0), (42.0, 24.0)]; // (x center, top y)
    let mut rgba = vec![0u8; N * N * 4];
    for y in 0..N {
        for x in 0..N {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let cx = fx.clamp(RADIUS, N as f32 - RADIUS);
            let cy = fy.clamp(RADIUS, N as f32 - RADIUS);
            if (fx - cx).powi(2) + (fy - cy).powi(2) > RADIUS * RADIUS {
                continue; // outside the rounded square: transparent
            }
            let t = fy / N as f32;
            let mut px =
                [(124.0 - 34.0 * t) as u8, (92.0 - 32.0 * t) as u8, 255u8.saturating_sub((40.0 * t) as u8), 255];
            if BARS.iter().any(|&(bx, top)| (fx - bx).abs() <= 4.0 && (top..=52.0).contains(&fy)) {
                px = [255, 255, 255, 255];
            }
            let i = (y * N + x) * 4;
            rgba[i..i + 4].copy_from_slice(&px);
        }
    }
    IconData { rgba, width: N as u32, height: N as u32 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn icon_is_64x64_rgba_with_transparent_corners() {
        let icon = super::app_icon();
        assert_eq!((icon.width, icon.height), (64, 64));
        assert_eq!(icon.rgba.len(), 64 * 64 * 4);
        assert_eq!(icon.rgba[3], 0, "canto deve ser transparente");
    }
}
