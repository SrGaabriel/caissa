# Caissa

![Example board](/web/static/brand/background_board.png)
> Note: piece assets are from https://chess.com/ and sound effects from https://lichess.org/

Move calculating checklist:
- [x] PAWN capture
- [x] PAWN march
- [x] BISHOP cross
- [x] KNIGHT jump
- [x] ROOK slide
- [x] QUEEN slide+cross
- [x] KING one-ranged slide+cross
- [x] En passant
- [x] Check coercion
- [x] Check dodging
- [ ] Promotion (only to queen)
- [x] Castling
> Disclaimer: move generation code is currently very repetitive for performance reasons, refactor coming soon