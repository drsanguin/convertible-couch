use std::mem::size_of;

use windows::Win32::Media::Audio::{waveOutGetDevCapsW, waveOutGetNumDevs, WAVEOUTCAPSW};

pub struct AudioSettings {}

impl AudioSettings {
    pub fn do_something() {
        unsafe {
            let num_devs_as_u32 = Self::wave_out_get_num_devs();
            let num_devs = usize::try_from(num_devs_as_u32).unwrap();

            (0..num_devs).for_each(|num_dev| {
                let mut caps = WAVEOUTCAPSW::default();
                let cbwoc_as_usize = size_of::<WAVEOUTCAPSW>();
                let cbwoc = u32::try_from(cbwoc_as_usize).unwrap();

                let wave_out_get_dev_caps_w_result =
                    Self::wave_out_get_dev_caps_w(num_dev, &mut caps, cbwoc);

                let sz_pname = caps.szPname;
                let sz_pname_as_string = String::from_utf16(&sz_pname).unwrap();

                let w_m_id = caps.wMid;
                let w_p_id = caps.wPid;

                println!("num_dev {num_dev} = {{ {sz_pname_as_string} {w_m_id} {w_p_id} }} result = {wave_out_get_dev_caps_w_result}");
            });
        }
    }

    unsafe fn wave_out_get_dev_caps_w(
        udeviceid: usize,
        pwoc: *mut WAVEOUTCAPSW,
        cbwoc: u32,
    ) -> u32 {
        waveOutGetDevCapsW(udeviceid, pwoc, cbwoc)
    }

    unsafe fn wave_out_get_num_devs() -> u32 {
        waveOutGetNumDevs()
    }
}
