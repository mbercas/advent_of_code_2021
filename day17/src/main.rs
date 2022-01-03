
#[derive(Debug)]
struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Target {
    fn new( x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> Target {
        Target{ x_min: x_min,
                x_max: x_max,
                y_min: y_min,
                y_max: y_max }
    }

    fn is_inside(&self, state: &State) -> bool {
        (state.x >= self.x_min && state.x <= self.x_max) && (state.y >= self.y_min && state.y <= self.y_max)
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl PartialEq for State {

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.vx == other.vx && self.vy == other.vy
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y || self.vx != other.vx || self.vy == other.vy
    }
}

impl State {

    fn new(x: i64, y: i64, vx: i64, vy: i64) -> State {
        State {
            x: x,
            y: y,
            vx: vx,
            vy: vy,
        }
    }

    fn step(&self) -> State {
        let new_vx = if self.vx.abs() > 0 { self.vx -1*self.vx.signum() } else { 0_i64 };

        State{
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: new_vx,
            vy: self.vy - 1,
        }
    }
}

type Trajectory = Vec<State>;

fn make_trajectory(vx: i64, vy: i64, target: &Target) -> Trajectory {
    let mut vec = Trajectory::new();
    let mut state = State::new(0, 0, vx, vy);
    vec.push(state);
    loop {
        if (state.x > target.x_max)
            || (state.x < target.x_min && state.vx == 0)
            || (state.y < target.y_min)
        {
            break
        }
        state = vec.last().unwrap().step();
        vec.push(state);
    }
    vec
}

fn is_trajectory_in_target(traj: &Trajectory, target: &Target) -> bool {
    for state in traj {
        if target.is_inside(state) {
            return true;
        }
    }
    return false;
}

fn find_highest_shooting_position(target: &Target) -> i64 {
    let max_vx = ((target.x_max.pow(2) + target.y_min.pow(2)) as f64).sqrt() as i64;
    let min_vy = -((target.x_min.pow(2) + target.y_min.pow(2)) as f64).sqrt() as i64;
    let mut y_max: i64 = 0;
    for vx in 1..=max_vx {
        for vy in min_vy..-1*target.y_min {
            let traj = make_trajectory(vx, vy, target);
            if is_trajectory_in_target(&traj, target) {
                let tmp_y_max = traj.iter().map(|s| s.y).max().unwrap();
                if tmp_y_max >= y_max {
                    y_max = tmp_y_max;
                }
            }
        }
    }
    y_max
}


fn count_good_shooting_position(target: &Target) -> u64 {
    let max_vx = ((target.x_max.pow(2) + target.y_min.pow(2)) as f64).sqrt() as i64;
    let min_vy = -((target.x_min.pow(2) + target.y_min.pow(2)) as f64).sqrt() as i64;
    let mut counter: u64 = 0;
    for vx in 1..=max_vx {
        for vy in min_vy..-1*target.y_min {
            let traj = make_trajectory(vx, vy, target);
            if is_trajectory_in_target(&traj, target) {
                counter += 1;
            }
        }
    }
    counter
}


fn main() {
    println!("x=211..232, y=-124..-69");
    let target = Target::new(211, 232, -124, -69);
    let y_max = find_highest_shooting_position(&target);
    println!("Highest shooting height is; {}", y_max);
    let counter = count_good_shooting_position(&target);
    println!("Good shooting positions: {}", counter);
}


#[cfg(test)]
mod test {

    use super::*;



    #[test]
    fn test_count_good_shooting_position() {
        let target = Target::new(20, 30, -10, -5);
        let counter = count_good_shooting_position(&target);
        assert_eq!(112, counter);
    }


    #[test]
    fn test_find_highest_shooting_position() {
        let target = Target::new(20, 30, -10, -5);
        let y_max = find_highest_shooting_position(&target);
        assert_eq!(45, y_max);
    }

    #[test]
    fn test_is_trajectory_in_target() {
        let target = Target::new(20, 30, -10, -5);

        let traj = make_trajectory(7, 2, &target);
        assert_eq!(true, is_trajectory_in_target(&traj, &target));

        let traj = make_trajectory(6, 3, &target);
        assert_eq!(true, is_trajectory_in_target(&traj, &target));

        let traj = make_trajectory(9, 0, &target);
        assert_eq!(true, is_trajectory_in_target(&traj, &target));

        let traj = make_trajectory(17, 04, &target);
        assert_eq!(false, is_trajectory_in_target(&traj, &target));

    }

    #[test]
    fn test_make_trajectory() {
        let target = Target::new(211, 232, -124, -69);
        let traj = make_trajectory(200, 0, &target);

        assert_eq!(3, traj.len());
        assert_eq!(Some(&State::new(0, 0, 200, 0)), traj.get(0));
        assert_eq!(Some(&State::new(200, 0, 199, -1)), traj.get(1));
        assert_eq!(Some(&State::new(399, -1, 198, -2)), traj.get(2));

        let traj = make_trajectory(0, -10, &target);
        assert_eq!(1, traj.len());
        assert_eq!(Some(&State::new(0, 0, 0, -10)), traj.get(0));

        let traj = make_trajectory(10, -100, &target);
        assert_eq!(3, traj.len());
        assert_eq!(Some(&State::new(0, 0, 10, -100)), traj.get(0));
        assert_eq!(Some(&State::new(10, -100, 9, -101)), traj.get(1));
        assert_eq!(Some(&State::new(19, -201, 8, -102)), traj.get(2));

    }

    #[test]
    fn test_state_step() {
        let state = State::new(0, 0, 0, 0);
        let mut new_state = state.step();

        assert_eq!(State::new(0, 0, 0, -1), new_state);
        new_state = new_state.step();
        assert_eq!(State::new(0, -1, 0, -2), new_state);
        new_state = new_state.step();
        assert_eq!(State::new(0, -3, 0, -3), new_state);

        let state = State::new(10, 0, 5, 0);
        let mut new_state = state.step();
        assert_eq!(State::new(15, 0, 4, -1), new_state);
        new_state = new_state.step();
        assert_eq!(State::new(19, -1, 3, -2), new_state);
    }
}
