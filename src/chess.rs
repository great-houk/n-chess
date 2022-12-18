use bevy::{prelude::*, utils::HashMap};
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

const CHESS_LIGHT_COLOR: Color = Color::rgb(
    0xFF as f32 / 0xFF as f32,
    0xCE as f32 / 0xFF as f32,
    0x9F as f32 / 0xFF as f32,
);
const CHESS_DARK_COLOR: Color = Color::rgb(
    0xD2 as f32 / 0xFF as f32,
    0x8A as f32 / 0xFF as f32,
    0x44 as f32 / 0xFF as f32,
);

pub struct Chess {
    pub number: usize,
}

impl Plugin for Chess {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(self.number))
            .add_startup_system(draw_board);
    }
}

#[derive(Debug, Resource)]
struct Board {
    players: Vec<Player>,
    grid: HashMap<Location, OwnedPiece>,
}

impl Board {
    fn new(count: usize) -> Self {
        let mut grid = HashMap::with_capacity(16 * count);
        let mut players = Vec::with_capacity(count);
        for p in 0..count {
            // Insert Player
            players.push(Player(p));
            // Place Pawns
            for x in 0..4 {
                grid.insert(
                    Location(Player(p), Half::Left, Coord(x, 1)),
                    OwnedPiece(Player(p), Piece::Pawn),
                );
                grid.insert(
                    Location(Player(p), Half::Right, Coord(x, 1)),
                    OwnedPiece(Player(p), Piece::Pawn),
                );
            }
            // Place Rook
            grid.insert(
                Location(Player(p), Half::Left, Coord(0, 1)),
                OwnedPiece(Player(p), Piece::Rook),
            );
            grid.insert(
                Location(Player(p), Half::Right, Coord(3, 1)),
                OwnedPiece(Player(p), Piece::Rook),
            );
            // Place Knight
            grid.insert(
                Location(Player(p), Half::Left, Coord(1, 1)),
                OwnedPiece(Player(p), Piece::Knight),
            );
            grid.insert(
                Location(Player(p), Half::Right, Coord(2, 1)),
                OwnedPiece(Player(p), Piece::Knight),
            );
            // Place Bishop
            grid.insert(
                Location(Player(p), Half::Left, Coord(2, 1)),
                OwnedPiece(Player(p), Piece::Bishop),
            );
            grid.insert(
                Location(Player(p), Half::Right, Coord(1, 1)),
                OwnedPiece(Player(p), Piece::Bishop),
            );
            // Place King/Queen
            grid.insert(
                Location(Player(p), Half::Left, Coord(3, 1)),
                OwnedPiece(Player(p), Piece::Queen),
            );
            grid.insert(
                Location(Player(p), Half::Right, Coord(0, 1)),
                OwnedPiece(Player(p), Piece::King),
            );
        }
        Board { players, grid }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location(Player, Half, Coord);

impl Location {
    fn corners(&self, num_players: usize) -> [Vec2; 4] {
        let side_length2 = (std::f32::consts::PI / (num_players * 2) as f32).tan();
        let top_right = Vec2::ZERO;
        let bott_right = Vec2::new(0.0, -1.0);
        let (bott_left, top_left) = match self.1 {
            Half::Left => (
                Vec2::new(-side_length2, -1.0),
                Vec2::new(
                    -(std::f32::consts::PI / num_players as f32).sin(),
                    -(std::f32::consts::PI / num_players as f32).cos(),
                ),
            ),
            Half::Right => (
                Vec2::new(side_length2, -1.0),
                Vec2::new(
                    (std::f32::consts::PI / num_players as f32).sin(),
                    -(std::f32::consts::PI / num_players as f32).cos(),
                ),
            ),
        };
        let xl = self.2 .0 as f32 * 0.25;
        let xh = (self.2 .0 + 1) as f32 * 0.25;
        let yl = self.2 .1 as f32 * 0.25;
        let yh = (self.2 .1 + 1) as f32 * 0.25;
        let rot = {
            let rad = 2.0 * self.0 .0 as f32 * std::f32::consts::PI / num_players as f32;
            Vec2::new(rad.cos(), rad.sin())
        };
        [
            rot.rotate(
                bott_right
                    .lerp(bott_left, xl)
                    .lerp(top_right.lerp(top_left, xl), yl),
            ),
            rot.rotate(
                bott_right
                    .lerp(bott_left, xl)
                    .lerp(top_right.lerp(top_left, xl), yh),
            ),
            rot.rotate(
                bott_right
                    .lerp(bott_left, xh)
                    .lerp(top_right.lerp(top_left, xh), yh),
            ),
            rot.rotate(
                bott_right
                    .lerp(bott_left, xh)
                    .lerp(top_right.lerp(top_left, xh), yl),
            ),
        ]
    }

    fn go(&self, path: &[Dir], num_players: usize) -> Option<Self> {
        let mut ret = self.clone();
        for m in path {
            match m {
                Dir::U => {
                    if ret.2 .1 == 3 {
                        (ret.0, ret.1) = match ret.1 {
                            Half::Left => (Player(ret.0 .0 + 1 % num_players), Half::Right),
                            Half::Right => (
                                Player(if ret.0 .0 == 0 {
                                    num_players - 1
                                } else {
                                    ret.0 .0 - 1
                                }),
                                Half::Left,
                            ),
                        };
                    } else {
                        ret.2 .1 += 1;
                    }
                }
                Dir::R => match ret.1 {
                    Half::Left => {
                        if ret.2 .0 == 0 {
                            ret.1 = Half::Right
                        } else {
                            ret.2 .0 -= 1
                        }
                    }
                    Half::Right => {
                        if ret.2 .0 == 3 {
                            return None;
                        } else {
                            ret.2 .0 += 1
                        }
                    }
                },
                Dir::L => match ret.1 {
                    Half::Left => {
                        if ret.2 .0 == 3 {
                            return None;
                        } else {
                            ret.2 .0 += 1
                        }
                    }
                    Half::Right => {
                        if ret.2 .0 == 0 {
                            ret.1 = Half::Left
                        } else {
                            ret.2 .0 -= 1
                        }
                    }
                },
                Dir::D => {
                    if ret.2 .1 == 0 {
                        return None;
                    } else {
                        ret.2 .1 -= 1;
                    }
                }
            }
        }
        Some(ret)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Half {
    Left,
    Right,
}

impl Half {
    fn get_points(&self, number: usize) -> [[Vec2; 5]; 5] {
        let side_length2 = (std::f32::consts::PI / (number * 2) as f32).tan();
        let top_right = Vec2::ZERO;
        let bott_right = Vec2::new(0.0, -1.0);
        let (bott_left, top_left) = match self {
            Half::Left => (
                Vec2::new(-side_length2, -1.0),
                Vec2::new(
                    -(std::f32::consts::PI / number as f32).sin(),
                    -(std::f32::consts::PI / number as f32).cos(),
                ),
            ),
            Half::Right => (
                Vec2::new(side_length2, -1.0),
                Vec2::new(
                    (std::f32::consts::PI / number as f32).sin(),
                    -(std::f32::consts::PI / number as f32).cos(),
                ),
            ),
        };
        let mut ret = [[Vec2::ZERO; 5]; 5];
        for y in 0..=4 {
            for x in 0..=4 {
                let xl = x as f32 * 0.25;
                let yl = y as f32 * 0.25;
                ret[x][y] = bott_right
                    .lerp(bott_left, xl)
                    .lerp(top_right.lerp(top_left, xl), yl);
            }
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord(i8, i8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OwnedPiece(Player, Piece);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn draw_board(mut commands: Commands, board: Res<Board>) {
    const CHECKER_POINT_ORDER: [(usize, usize); 33] = [
        (0, 0),
        (1, 0),
        (1, 1),
        (2, 1),
        (2, 0),
        (3, 0),
        (3, 1),
        (4, 1),
        (4, 2),
        (3, 2),
        (3, 3),
        (4, 3),
        (4, 4),
        (3, 4),
        (3, 3),
        (2, 3),
        (2, 4),
        (1, 4),
        (1, 3),
        (0, 3),
        (0, 2),
        (1, 2),
        (1, 3),
        (2, 3),
        (2, 2),
        (3, 2),
        (3, 1),
        (2, 1),
        (2, 2),
        (1, 2),
        (1, 1),
        (0, 1),
        (0, 0),
    ];
    let radius = 4.5;
    let outline = shapes::RegularPolygon {
        sides: board.players.len() * 2,
        feature: shapes::RegularPolygonFeature::Apothem(radius),
        center: Vec2::new(0., 0.),
    };
    let mut shapes = Vec::with_capacity(board.players.len() + 2);
    // Push dark board fill
    shapes.push(GeometryBuilder::build_as(
        &outline,
        DrawMode::Fill(FillMode::color(CHESS_DARK_COLOR)),
        Default::default(),
    ));
    // Push all the light squares
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);
    for _ in 0..board.players.len() {
        let left = Half::Left.get_points(board.players.len());
        let right = Half::Right.get_points(board.players.len());
        shapes.push(GeometryBuilder::build_as(
            &Polygon {
                points: Vec::from(CHECKER_POINT_ORDER.map(|(x, y)| left[x][y] * radius)),
                closed: true,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(CHESS_LIGHT_COLOR),
                outline_mode: StrokeMode::new(Color::BLACK, 0.03),
            },
            transform,
        ));
        shapes.push(GeometryBuilder::build_as(
            &Polygon {
                points: Vec::from(CHECKER_POINT_ORDER.map(|(x, y)| right[4 - x][y] * radius)),
                closed: true,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(CHESS_LIGHT_COLOR),
                outline_mode: StrokeMode::new(Color::BLACK, 0.03),
            },
            transform,
        ));
        transform.rotate_z(2.0 * std::f32::consts::PI / board.players.len() as f32);
    }
    // Push board outline
    shapes.push(GeometryBuilder::build_as(
        &outline,
        DrawMode::Stroke(StrokeMode::new(Color::BLACK, 0.05)),
        Transform::from_xyz(0., 0., 0.),
    ));
    // Draw Board
    commands.spawn_batch(shapes);
}

fn draw_pieces(mut commands: Commands, board: Res<Board>) {}
