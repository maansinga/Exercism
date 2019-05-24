#[derive(Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl ToString for Clock{
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Clock {
    fn recalc(hours: i32, minutes: i32) -> (i32, i32){
        let recalc_minutes = (60 + minutes % 60) % 60;
        let delta_hours = if minutes >= 0 || minutes % 60 == 0 {
            minutes / 60
        }else{
            (minutes - 60)/ 60
        };
        let new_hours = (24 + (delta_hours + hours) % 24) % 24;


        (new_hours, recalc_minutes)
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (rh, m) = Clock::recalc(hours, minutes);

        Clock{hours: rh, minutes: m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let h = self.hours;
        let m = self.minutes + minutes;

        Clock::new(h,  m)
    }
}
