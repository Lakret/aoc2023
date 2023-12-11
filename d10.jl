using Pipe

parse_input(input) = @pipe input |> strip |> split(_, "\n") |> collect.(_) |> mapreduce(permutedims, vcat, _)

CONNECT_DELTAS = Dict(
  '.' => [],
  '|' => [CartesianIndex(-1, 0), CartesianIndex(1, 0)],
  '-' => [CartesianIndex(0, -1), CartesianIndex(0, 1)],
  'L' => [CartesianIndex(-1, 0), CartesianIndex(0, 1)],
  'J' => [CartesianIndex(-1, 0), CartesianIndex(0, -1)],
  '7' => [CartesianIndex(1, 0), CartesianIndex(0, -1)],
  'F' => [CartesianIndex(1, 0), CartesianIndex(0, 1)],
)
NEIGHBOUR_DELTAS = values(CONNECT_DELTAS) |> Iterators.flatten |> Set |> collect
PIPE_SHAPES = @pipe keys(CONNECT_DELTAS) |> filter(x -> x != '.', _) |> collect

isvalid(tiles, pos::CartesianIndex{2})::Bool =
  pos[1] >= 1 && pos[2] >= 1 && pos[1] <= size(tiles)[1] && pos[2] <= size(tiles)[2]

"Finds cartesian coordinates of tiles that a pipe at `pos` connects"
function connects(tiles, pos::CartesianIndex{2}; shape=nothing)::Vector{CartesianIndex{2}}
  shape = isnothing(shape) ? tiles[pos] : shape

  [pos + delta for delta in CONNECT_DELTAS[shape] if isvalid(tiles, pos + delta)]
end

"Checks if a tile at `pos1` contains a pipe connected with the pipe at `pos2` tile."
function isconnected(tiles, pos1::CartesianIndex{2}, pos2::CartesianIndex{2}; pos1_shape=Nothing)::Bool
  pos2 in connects(tiles, pos1; shape=pos1_shape) && pos1 in connects(tiles, pos2)
end

"Returns a vector of cartesian coordinates of neighbouring tiles (disregarding pipe connections)"
neighbours(tiles, pos) = [pos + delta for delta in NEIGHBOUR_DELTAS if isvalid(tiles, pos + delta)]

function start_pos_and_shape(tiles)
  pos = findfirst(x -> x == 'S', tiles)

  for shape in PIPE_SHAPES
    if [isconnected(tiles, pos, n_pos; pos1_shape=shape) for n_pos in neighbours(tiles, pos)] |> sum == 2
      return (pos, shape)
    end
  end
end

function loop_steps(tiles; first_neighbour_idx=1)
  (s_pos, s_shape) = start_pos_and_shape(tiles)
  tiles = deepcopy(tiles)
  tiles[s_pos] = s_shape

  distances = Dict(s_pos => 0)
  pos = connects(tiles, s_pos)[first_neighbour_idx]
  steps = 1
  distances[pos] = steps

  while true
    next_pos = [next_pos for next_pos in connects(tiles, pos) if next_pos ∉ keys(distances)]
    if isempty(next_pos)
      return distances
    else
      pos = first(next_pos)
      steps += 1
      distances[pos] = steps
    end
  end
end

function p1(tiles)
  distances = loop_steps(tiles)
  backward_distances = loop_steps(tiles; first_neighbour_idx=2)

  for (pos, dist) in distances
    if backward_distances[pos] == dist && dist != 0
      return dist
    end
  end
end

function clean_tiles(tiles)
  tiles = deepcopy(tiles)
  loop_tiles = loop_steps(tiles) |> keys |> Set
  s_pos, s_shape = start_pos_and_shape(tiles)
  tiles[s_pos] = s_shape
  for pos in CartesianIndices(tiles)
    if pos ∉ loop_tiles
      tiles[pos] = '.'
    end
  end

  (loop_tiles, tiles)
end

# based on even-odd rule + additional rules for the "portal" shape L-J, F-7 (don't count them)
# and crossings L-7 and F-J (count them)
# https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
function p2(tiles)
  (loop_tiles, tiles) = clean_tiles(tiles)
  outside = fill(false, size(tiles))

  for row in 1:size(tiles)[1]
    angle_pipe_up = nothing
    inside = false

    for col in 1:size(tiles)[2]
      tile = tiles[row, col]

      if tile == '|'
        inside = !inside
      elseif tile == 'L'
        angle_pipe_up = true
      elseif tile == 'F'
        angle_pipe_up = false
      elseif tile == 'J'
        if !angle_pipe_up
          inside = !inside
        end

        angle_pipe_up = nothing
      elseif tile == '7'
        if angle_pipe_up
          inside = !inside
        end

        angle_pipe_up = nothing
      end

      if !inside
        outside[row, col] = true
      end
    end
  end

  for loop_tile in loop_tiles
    outside[loop_tile] = true
  end

  (size(tiles) |> prod) - sum(outside)
end

test_tiles = parse_input(
  """
  ..F7.
  .FJ|.
  SJ.L7
  |F--J
  LJ...
  """
)

tiles = readchomp("inputs/d10") |> parse_input

@assert start_pos_and_shape(test_tiles) == (CartesianIndex(3, 1), 'F')
@assert start_pos_and_shape(tiles) == (CartesianIndex(37, 109), '7')

@assert p1(test_tiles) == 8
@time @assert @show p1(tiles) == 6768

@assert p2(test_tiles) == 1

test_tiles2 = parse_input(
  """
  ...........
  .S-------7.
  .|F-----7|.
  .||.....||.
  .||.....||.
  .|L-7.F-J|.
  .|..|.|..|.
  .L--J.L--J.
  ...........
  """
)
@assert p2(test_tiles2) == 4

test_tiles3 = parse_input(
  """
  ..........
  .S------7.
  .|F----7|.
  .||....||.
  .||....||.
  .|L-7F-J|.
  .|..||..|.
  .L--JL--J.
  ..........
  """
)
@assert p2(test_tiles3) == 4

test_tiles4 = parse_input(
  """
  .F----7F7F7F7F-7....
  .|F--7||||||||FJ....
  .||.FJ||||||||L7....
  FJL7L7LJLJ||LJ.L-7..
  L--J.L7...LJS7F-7L7.
  ....F-J..F7FJ|L7L7L7
  ....L7.F7||L7|.L7L7|
  .....|FJLJ|FJ|F7|.LJ
  ....FJL-7.||.||||...
  ....L---J.LJ.LJLJ...
  """
)
@assert p2(test_tiles4) == 8

test_tiles5 = parse_input(
  """
  FF7FSF7F7F7F7F7F---7
  L|LJ||||||||||||F--J
  FL-7LJLJ||||||LJL-77
  F--JF--7||LJLJ7F7FJ-
  L---JF-JLJ.||-FJLJJ7
  |F|F-JF---7F7-L7L|7|
  |FFJF7L7F-JF7|JL---7
  7-L-JL7||F7|L7F-7F7|
  L.L7LFJ|||||FJL7||LJ
  L7JLJL-JLJLJL--JLJ.L
  """
)
@assert p2(test_tiles5) == 10

@time @show @assert p2(tiles) == 351
