using Pipe

parse_input(input) = @pipe input |> strip |> split(_, "\n") |> collect.(_) |> mapreduce(permutedims, vcat, _)

@enum Direction up down left right

function move(beam)
  (heading_to, direction) = beam

  if direction == up
    (heading_to + CartesianIndex(-1, 0), direction)
  elseif direction == down
    (heading_to + CartesianIndex(1, 0), direction)
  elseif direction == left
    (heading_to + CartesianIndex(0, -1), direction)
  elseif direction == right
    (heading_to + CartesianIndex(0, 1), direction)
  end
end

function reflect(beam, tile)
  (heading_to, direction) = beam

  new_direction =
    if direction == right && tile == '/'
      up
    elseif direction == left && tile == '/'
      down
    elseif direction == up && tile == '/'
      right
    elseif direction == down && tile == '/'
      left
    elseif direction == right && tile == '\\'
      down
    elseif direction == left && tile == '\\'
      up
    elseif direction == up && tile == '\\'
      left
    elseif direction == down && tile == '\\'
      right
    end

  (heading_to, new_direction)
end

function isinside(beam, grid)
  (heading_to, _direction) = beam
  heading_to[1] >= 1 && heading_to[2] >= 1 && heading_to[1] <= size(grid)[1] && heading_to[2] <= size(grid)[2]
end

function simulate(grid, start=(CartesianIndex(1, 1), right))
  energized = falses(size(grid)...)
  beams = [start]
  seen = Set([start])

  while !isempty(beams)
    beam = pop!(beams)
    push!(seen, beam)

    (heading_to, direction) = beam
    energized[heading_to] = true
    tile = grid[heading_to]

    new_beams =
      if tile == '.' || (tile == '-' && direction ∈ [left, right]) || (tile == '|' && direction ∈ [up, down])
        [move(beam)]
      elseif tile == '/' || tile == '\\'
        [reflect(beam, tile) |> move]
      elseif tile == '-'
        move.([(heading_to, left), (heading_to, right)])
      elseif tile == '|'
        move.([(heading_to, up), (heading_to, down)])
      end

    @pipe filter(beam -> isinside(beam, grid) && beam ∉ seen, new_beams) |> append!(beams, _)
  end

  energized
end

p1(grid) = simulate(grid) |> sum

function all_starts(grid)
  starts = [(CartesianIndex(1, col), down) for col in 1:size(grid)[2]]
  append!(starts, [(CartesianIndex(size(grid)[1], col), up) for col in 1:size(grid)[2]])
  append!(starts, [(CartesianIndex(row, 1), right) for row in 1:size(grid)[1]])
  append!(starts, [(CartesianIndex(row, size(grid)[2]), left) for row in 1:size(grid)[1]])

  starts
end

p2(grid) = maximum(start -> simulate(grid, start) |> sum, all_starts(grid))

test_grid = parse_input(
  raw"""
  .|...\....
  |.-.\.....
  .....|-...
  ........|.
  ..........
  .........\
  ..../.\\..
  .-.-/..|..
  .|....-|.\
  ..//.|....
  """
)
grid = readchomp("inputs/d16") |> parse_input

(_beams, energized) = simulate(test_grid)
@assert p1(test_grid) == 46
@time @assert @show p1(grid) == 7496

@assert p2(test_grid) == 51
@time @show p2(grid) == 7932
