using Pipe

parse_input(input) = @pipe input |> strip |> split(_, "\n") |> collect.(_) |> mapreduce(permutedims, vcat, _)

function tilt_north(input)
  tilted = fill('.', size(input))

  for col = 1:size(tilted)[2]
    carrying = []
    for row = (size(input)[1]):-1:1
      if input[row, col] == '#'
        tilted[row, col] = '#'
        offset = 1
        while !isempty(carrying)
          round_rock_row = pop!(carrying)
          tilted[round_rock_row, col] = '.'
          tilted[row+offset, col] = 'O'
          offset += 1
        end
      elseif input[row, col] == 'O'
        push!(carrying, row)
      end
    end

    offset = 1
    while !isempty(carrying)
      round_rock_row = pop!(carrying)
      tilted[round_rock_row, col] = '.'
      tilted[offset, col] = 'O'
      offset += 1
    end
  end

  tilted
end

function p1(input)
  tilted = tilt_north(input)
  max_row = size(tilted)[1]
  [max_row - pos[1] + 1 for pos in findall(x -> x == 'O', tilted)] |> sum
end

cycle(input) =
  input |> tilt_north |> rotr90 |> tilt_north |> rotr90 |> tilt_north |> rotr90 |> tilt_north |> rotr90

function p2(input)
  tilted = cycle(input)
  tilted
end

test_input = parse_input(
  """
  O....#....
  O.OO#....#
  .....##...
  OO.#O....O
  .O.....O#.
  O.#..O.#.#
  ..O..#O..O
  .......O..
  #....###..
  #OO..#....
  """
)

input = readchomp("inputs/d14") |> parse_input

@assert p1(test_input) == 136
@time @show p1(input) == 110090