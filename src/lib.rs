pub static DELTA_TIME: f32 = 0.0166667;

pub fn effective_delta_time(time_active: f32)->f32{
    (time_active+DELTA_TIME)-time_active
}
static FRAME_REGIMES: [usize;26] = [0,1,3,7,14,29,59,119,240,479,960,1920,3840,7679,15361,30724,61452,122683,246044,492768,986216,1918283,4015435,8209739,16598346,24986954];
/// Computes the number of frames until spinner freeze occurs
/// Takes in the current time active and returns the number of frames
/// # Example
/// ```
/// use spin_freeze::frames_to_freeze;
/// 
/// assert_eq!(frames_to_freeze(0.0),24986955);
/// ```
pub fn frames_to_freeze(time_active: f32)->usize{
    if time_active == 0.0{
        return 1+frames_to_freeze(DELTA_TIME);
    }
    let next_pow = f32::log2(time_active).ceil();
    if time_active==next_pow{
        24986954-FRAME_REGIMES[(next_pow + 6.0) as usize]
    } else {
        let mut frames = 0;
        let mut time_active = time_active;
        let target = f32::powi(2.0, next_pow as i32);
        while time_active< target{
            frames+=1;
            time_active+=DELTA_TIME;
        }
        frames+24986954-FRAME_REGIMES[(next_pow + 6.0) as usize]
    }
}
static TA_BEFORE_FREEZE: f32 = 524288.0-0.03125;
pub fn freeze_comands(time_active: f32, chapter_time: usize, frames_before_freeze: usize)->String{
    let frames_to_wait = frames_to_freeze(time_active) - frames_before_freeze;
    let mut target_ta= if frames_before_freeze == 0{
        524288.0
    } else {
        TA_BEFORE_FREEZE
    };
    for _ in 1..frames_before_freeze{
        target_ta-=DELTA_TIME;
    }
    let final_chapter_time = frames_to_wait+chapter_time;
    format!("#console evalcs SavaData.Instance.AddTime((Monocle.Engine.Scene as Level).Session.Area, {}*170000l)\nSet, Level.TimeActive, {:.5}\nSet, Session.Time, 170000*{}",
        frames_to_wait, target_ta,final_chapter_time)
}
/// Computes the largest number of cycles which can occur before freeze
/// Returns the number of cycles and the left over number of frames
pub fn cycles_to_freeze(time_active: f32,cycle_length: usize)->(usize,usize){
   let freeze = frames_to_freeze(time_active);
   let cycles = freeze/cycle_length;
   let rem = freeze-(cycles*cycle_length);
   (cycles,rem) 
}
#[derive(Debug,PartialEq)]
pub struct CycleInfo{
    pub cycle_count: usize,
    time_active: f32,
    frames_elapsed: usize,
    final_chapter_time: usize,
    pub remaining_frames: usize
}
pub fn get_cycle_wait_info(time_active: f32,cycle_length: usize,chapter_time: usize,frames_before_freeze: usize)->CycleInfo{
    let (mut cycles_to_wait,mut rem) = cycles_to_freeze(time_active, cycle_length);
    //cycles to wait: 290,466
    //rem: 45
    let mut bef_mult = frames_before_freeze/cycle_length;
    //bef_mult: // 5
    let bef_rem = frames_before_freeze - (bef_mult*cycle_length);
    //bef_rem // 33
    if bef_rem>rem{ 
        rem+=cycle_length;
        bef_mult+=1;
        dbg!(cycle_length);
    }
    cycles_to_wait-=bef_mult;
    rem-=bef_rem;
    let frames_to_wait = cycles_to_wait*cycle_length;
    let frames_before_freeze = frames_before_freeze+rem;
    let mut target_ta= if frames_before_freeze == 0{
        524288.0
    } else {
        TA_BEFORE_FREEZE
    };
    for _ in 1..frames_before_freeze{
        target_ta-=DELTA_TIME;
    }
    let final_chapter_time = frames_to_wait+chapter_time;
    CycleInfo { cycle_count: cycles_to_wait, time_active: target_ta, frames_elapsed: frames_to_wait, final_chapter_time, remaining_frames: rem}
}
pub fn cycle_commands(time_active: f32,cycle_length: usize,chapter_time: usize,frames_before_freeze: usize)->(String,String){
    let info =get_cycle_wait_info(time_active,cycle_length,chapter_time,frames_before_freeze);
    (format!("#console evalcs SavaData.Instance.AddTime((Monocle.Engine.Scene as Level).Session.Area, {}*170000l)\nSet, Level.TimeActive, {:.5}\nSet, Session.Time, {}",
        info.frames_elapsed, info.time_active,170000*info.final_chapter_time),
    format!("And you must wait an additional {}f",info.remaining_frames))
}
#[cfg(test)]
mod tests{
    use crate::{cycles_to_freeze, frames_to_freeze, get_cycle_wait_info, DELTA_TIME};

    #[test]
    fn freeze_test(){
        let mut time_active = 0.0;
        for f in 0..65536{
            dbg!(f);
            assert_eq!(frames_to_freeze(time_active)+f,24986955);
            time_active+=DELTA_TIME;
        }
    }
    
    #[test]
    fn cycle_test(){
        let half_freeze = 24986954/2;
        assert_eq!(cycles_to_freeze(0.0, 2), (half_freeze,1));
        assert_eq!(cycles_to_freeze(DELTA_TIME,2), (half_freeze,0));
    }
    #[test]
    fn cycle_info(){
        let info = get_cycle_wait_info(0.0, 9, 0, 24);
        // we know there are 24986955f until freeze from 0.0
        // which means that the number of cycles that can fit is 2776328 with a remainder of 3f
        // 24f of waiting is 2 cycles with a remainder of 6f
        // so we can fit 2776325 cycles with a remainder of 6f before freeze
        assert_eq!(info.cycle_count,2776325);
        assert_eq!(info.remaining_frames,6);
        assert_eq!(info.frames_elapsed,24986925);//cycle count times 9
        assert_eq!(info.final_chapter_time,24986925);
        let mut ta = info.time_active;
        for _ in 0..(6+24){
            assert_ne!(ta,f32::powi(2.0, 19));
            ta+=DELTA_TIME;
        }
        assert_eq!(ta,f32::powi(2.0, 19));

        let info_2 = get_cycle_wait_info(DELTA_TIME, 9, 1, 24);
        assert_eq!(info_2.cycle_count,2776325);
        assert_eq!(info_2.remaining_frames,5);
        assert_eq!(info_2.frames_elapsed,24986925);//cycle count times 9
        assert_eq!(info_2.final_chapter_time,24986926);
        let mut ta = info_2.time_active;
        for _ in 0..(5+24){
            ta+=DELTA_TIME
        }
        assert_eq!(ta,f32::powi(2.0, 19));
        let mut time_active = 0.0;
        let mut chapter_time = 0;
        for _ in 0..6{
            time_active+=DELTA_TIME;
            chapter_time+=1;
        }
        let inf_t = get_cycle_wait_info(time_active, 9, chapter_time, 24);
        assert_eq!(inf_t.remaining_frames,0);
        let info_3 = get_cycle_wait_info(DELTA_TIME, 9, 1, 29);
        assert_eq!(info_3.cycle_count,2776325);
        assert_eq!(info_3.remaining_frames,0);
        assert_eq!(info_3.frames_elapsed,24986925);//cycle count times 9
        assert_eq!(info_3.final_chapter_time,24986926);
        let mut ta = info_3.time_active;
        for _ in 0..(29){
            ta+=DELTA_TIME
        }
        assert_eq!(ta,f32::powi(2.0, 19));
    }
}