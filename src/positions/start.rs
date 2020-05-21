use super::*;

impl Desk {
    pub fn start_position() -> Desk {
        Desk {cells: [
            Some(Figura{color: Color::White, figura: FiguraType::Rook}),
            Some(Figura{color: Color::White, figura: FiguraType::Knight}),
            Some(Figura{color: Color::White, figura: FiguraType::Bishop}),
            Some(Figura{color: Color::White, figura: FiguraType::Queen}),
            Some(Figura{color: Color::White, figura: FiguraType::King}),
            Some(Figura{color: Color::White, figura: FiguraType::Bishop}),
            Some(Figura{color: Color::White, figura: FiguraType::Knight}),
            Some(Figura{color: Color::White, figura: FiguraType::Rook}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::White, figura: FiguraType::Pawn}),
            None, None, None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None,
            None, None,
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Pawn}),
            Some(Figura{color: Color::Black, figura: FiguraType::Rook}),
            Some(Figura{color: Color::Black, figura: FiguraType::Knight}),
            Some(Figura{color: Color::Black, figura: FiguraType::Bishop}),
            Some(Figura{color: Color::Black, figura: FiguraType::Queen}),
            Some(Figura{color: Color::Black, figura: FiguraType::King}),
            Some(Figura{color: Color::Black, figura: FiguraType::Bishop}),
            Some(Figura{color: Color::Black, figura: FiguraType::Knight}),
            Some(Figura{color: Color::Black, figura: FiguraType::Rook})
                 ]
            , additional_figures: vec![]}
    }
    
    fn get_cells(&self) -> &[Option<Figura>] {
        &self.cells
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] //тестирование стартовой позиции
    fn test_start_desk() {
        
        let start_desk = Desk::start_position();
        let cells = start_desk.get_cells();

        //должно быть 64 клетки
        assert_eq!(cells.len(), 64);

        //должно быть 32 пустых клетки
        assert_eq!(cells.iter().filter(|fig| {
            match fig {
                None => true,
                _ => false
            }
        }).count(), 32);
        
        //должно быть 16 белых фигур
        assert_eq!(cells.iter().filter(|fig| {
            match fig {
                Some(fig2) => fig2.color == Color::White,
                None => false
            }
        }).count(), 16);
        
        //должно быть 2 короля
        assert_eq!(cells.iter().filter(|fig| {
            match fig {
                None => false,
                Some(fig2) => fig2.figura == FiguraType::King
            }
        }).count(), 2);
    }
}
