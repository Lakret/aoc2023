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
  beams = Set([start])
  energized = falses(size(grid)...)
  prev_beam_states = Set()

  while hash(beams) ∉ prev_beam_states
    push!(prev_beam_states, hash(beams))

    new_beams = Set()
    for beam in beams
      (heading_to, direction) = beam
      energized[heading_to] = true
      tile = grid[heading_to]

      if tile == '.' || (tile == '-' && direction ∈ [left, right]) || (tile == '|' && direction ∈ [up, down])
        beam = move(beam)

        if isinside(beam, grid)
          push!(new_beams, beam)
        end
      elseif tile == '/' || tile == '\\'
        beam = reflect(beam, tile) |> move

        if isinside(beam, grid)
          push!(new_beams, beam)
        end
      elseif tile == '-'
        @pipe (
          move.([(heading_to, left), (heading_to, right)])
          |> filter(beam -> isinside(beam, grid), _)
          |> push!(new_beams, _...)
        )
      elseif tile == '|'
        @pipe (
          move.([(heading_to, up), (heading_to, down)])
          |> filter(beam -> isinside(beam, grid), _)
          |> push!(new_beams, _...)
        )
      end
    end

    beams = new_beams
  end

  (beams, energized)
end

p1(grid) = simulate(grid)[2] |> sum

function all_starts(grid)
  starts = [(CartesianIndex(1, col), down) for col in 1:size(grid)[2]]
  append!(starts, [(CartesianIndex(size(grid)[1], col), up) for col in 1:size(grid)[2]])
  append!(starts, [(CartesianIndex(row, 1), right) for row in 1:size(grid)[1]])
  append!(starts, [(CartesianIndex(row, size(grid)[2]), left) for row in 1:size(grid)[1]])

  starts
end

function p2(grid)
  starts = all_starts(grid)
  energies = []

  Threads.@threads for start in starts
    push!(energies, simulate(grid, start)[2] |> sum)
  end

  energies |> maximum
end

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
@assert energized[1, :] == [1, 1, 1, 1, 1, 1, 0, 0, 0, 0]
@assert energized[2, :] == [0, 1, 0, 0, 0, 1, 0, 0, 0, 0]
@assert energized[3, :] == [0, 1, 0, 0, 0, 1, 1, 1, 1, 1]
@assert energized[8, :] == [1, 1, 1, 1, 1, 1, 1, 1, 0, 0]
@assert energized[10, :] == [0, 1, 0, 0, 0, 1, 0, 1, 0, 0]
@assert p1(test_grid) == 46
@time @assert @show p1(grid) == 7496

@time @assert p2(test_grid) == 51
@time @show p2(grid) == 7932

# with `julia --threads 24`:
# p2(grid) = 7932
#  86.304054 seconds (8.18 G allocations: 206.287 GiB, 69.33% gc time, 0.01% compilation time)
# 7932
