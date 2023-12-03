using Pipe

parse_input(inp) = @pipe split(inp) .|> collect |> mapreduce(permutedims, vcat, _)

neighbour_deltas = [
  CartesianIndex(-1, 0), CartesianIndex(1, 0), CartesianIndex(0, -1), CartesianIndex(0, 1),
  CartesianIndex(-1, -1), CartesianIndex(-1, 1), CartesianIndex(1, -1), CartesianIndex(1, 1)
]

function all_part_numbers(inp)
  part_numbers = []
  for i = 1:size(inp)[1]
    n = Vector{Char}()
    adjacent_to_symbol = false

    for j = 1:size(inp)[2]
      if isdigit(inp[i, j])
        push!(n, inp[i, j])

        idx = CartesianIndex(i, j)
        for delta = neighbour_deltas
          neighbour_idx = idx + delta
          if neighbour_idx[1] > 0 && neighbour_idx[2] > 0 &&
             neighbour_idx[1] < size(inp)[1] && neighbour_idx[2] < size(inp)[2]
            neighbour = inp[idx+delta]
            if !isdigit(neighbour) && neighbour != '.'
              adjacent_to_symbol = true
            end
          end
        end
      else
        if !isempty(n) && adjacent_to_symbol
          push!(part_numbers, parse(Int64, String(n)))
        end

        n = Vector{Char}()
        adjacent_to_symbol = false
      end
    end

    if !isempty(n) && adjacent_to_symbol
      push!(part_numbers, parse(Int64, String(n)))
    end
  end
  part_numbers
end

p1(inp) = all_part_numbers(inp) |> sum

test_inp = """467..114..
  ...*......
  ..35..633.
  ......#...
  617*......
  .....+.58.
  ..592.....
  ......755.
  ...\$.*....
  .664.598..
  """
test_inp = parse_input(test_inp)

input = readchomp("inputs/d03") |> parse_input

@assert p1(test_inp) == 4361
@assert @show p1(input) == 540212