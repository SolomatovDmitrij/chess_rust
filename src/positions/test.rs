use super::*;

impl Desk {
    pub fn position1() -> Desk {
        Desk {cells: [
            None, //0
            None, //1 
            None, //2
            None, //3
            None, //4
            None, //5
            None, //6
            None, //7
            None, //8
            None, //9
            None, //10
            None, //11
            None, //12
            None, //13
            None, //14
            None, //15
            None, //16
            None, //17
            None, //18
            None, //19
            None, //20
            None, //21
            None, //22
            Some(Figura {color: Color::Black, figura: FiguraType::Pawn}), //23
            None, //24
            None, //25
            None, //26
            None, //27
            None, //28
            None, //29
            None, //30
            None, //31
            None, //32
            Some(Figura {color: Color::White, figura: FiguraType::King}), //33
            Some(Figura {color: Color::White, figura: FiguraType::Pawn}), //34
            None, //35
            None, //36
            Some(Figura {color: Color::Black, figura: FiguraType::Rook}), //37
            None, //38
            None, //39
            None, //40
            None, //41
            None, //42
            None, //43
            None, //44
            None, //45
            None, //46
            None, //47
            None, //48
            None, //49
            None, //50
            None, //51
            None, //52
            None, //53
            None, //54
            Some(Figura {color: Color::Black, figura: FiguraType::Pawn}), //55
            None, //56
            None, //57
            Some(Figura {color: Color::Black, figura: FiguraType::King}), //58
            None, //59
            None, //60
            None, //61
            None, //62
            None  //63
                ]
                , additional_figures: vec![], color_move: Color::White}
    }
}

