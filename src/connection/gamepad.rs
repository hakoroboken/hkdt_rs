use gamepads::{Gamepads, Button};

#[derive(Debug, Clone, Copy)]
pub struct Axis
{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]

/// ボタン郡
pub struct Buttons
{
    pub circle : i8,
    pub triangle : i8,
    pub cube : i8,
    pub cross : i8,
    pub r1 : i8,
    pub r2 : i8,
    pub l1 : i8,
    pub l2 : i8
}

/// 取得したコントローラーデータを格納する
/// * `left_stick` - 左スティックの縦横データ(float)
/// * `right_stick` - 右スティックの縦横データ(float)
/// * `dpad` - 十字キーの縦横データ(float)
/// * `btns` - 各ボタンのデータ。(int)
#[derive(Debug, Clone, Copy)]
pub struct GamePadData
{
    pub left_stick : Axis,
    pub right_stick : Axis,
    pub dpad : Axis,
    pub btns : Buttons
}

impl Axis {
    pub fn new()->Self
    {
        Axis { x: 0.0, y: 0.0 }
    }
}
impl Buttons {
    pub fn new()->Self
    {
        Buttons { circle: 0, triangle: 0, cube: 0, cross: 0, r1: 0, r2: 0, l1: 0, l2: 0 }
    }
}

impl GamePadData {
    pub fn new()->Self
    {
        GamePadData { left_stick: Axis::new(), right_stick: Axis::new(), dpad: Axis::new(), btns: Buttons::new() }
    }
}
/// ドライバー本体
pub struct GamePadsDriver
{
    gamepads : Gamepads,
    data : Vec<GamePadData>
}
impl GamePadsDriver {
    /// 初期化関数
    pub fn new()->Self
    {
        Self { gamepads: Gamepads::new() , data : Vec::new()}
    }

    /// 受信データの更新を行う
    pub fn update(&mut self)
    {
        self.data.clear();
        self.gamepads.poll();

        for gamepad in self.gamepads.all()
        {
            let mut controller = GamePadData::new();

            controller.left_stick.x = gamepad.left_stick_x();
            controller.left_stick.y = gamepad.left_stick_y();
            if controller.left_stick.y.abs() < 0.05
            {
                controller.left_stick.y = 0.0;
            }
            controller.right_stick.x = gamepad.right_stick_x();
            controller.right_stick.y = gamepad.right_stick_y();

            controller.dpad.x = bool_to_f32(gamepad.is_currently_pressed(Button::DPadRight)) - bool_to_f32(gamepad.is_currently_pressed(Button::DPadLeft));
            controller.dpad.y = bool_to_f32(gamepad.is_currently_pressed(Button::DPadUp)) - bool_to_f32(gamepad.is_currently_pressed(Button::DPadDown));

            controller.btns.circle = bool_to_i8(gamepad.is_currently_pressed(Button::ActionRight));
            controller.btns.triangle = bool_to_i8(gamepad.is_currently_pressed(Button::ActionUp));
            controller.btns.cube = bool_to_i8(gamepad.is_currently_pressed(Button::ActionLeft));
            controller.btns.cross = bool_to_i8(gamepad.is_currently_pressed(Button::ActionDown));

            controller.btns.r1 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontRightUpper));
            controller.btns.r2 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontRightLower));
            controller.btns.l1 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontLeftUpper));
            controller.btns.l2 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontLeftLower));

            self.data.push(controller);     
        }
    }

    /// コントローラー数を取得する
    pub fn controller_num(&self)->usize
    {
        self.data.len()
    }

    /// コントローラーが接続されているか確認する
    /// * `true` - １台も接続されていない
    /// * `false` - 何台か接続されている
    pub fn is_connected(&self)->bool
    {
        if self.controller_num() == 0
        {
            false
        }
        else {
            true
        }
    }

    /// 指定された番号のコントローラーのデータを取得する
    /// * `id`  １台めのデータが欲しいなら「１」を入力する
    /// * `存在しない`台数目を指定された場合は空の構造体を返す
    pub fn get(&self, id : usize)->GamePadData
    {
        if id > self.controller_num() || id == 0
        {
            GamePadData::new()
        }
        else {
            self.data[id-1]
        }   
    }
}


fn bool_to_f32(b : bool)->f32
{
    if b
    {
        1.0
    }
    else 
    {
        0.0
    }
}

fn bool_to_i8(b: bool)->i8
{
    if b
    {
        1
    }
    else {
        0
    }
}