using Pipe

parse_input(inp) = @pipe split(inp) .|> collect |> mapreduce(permutedims, vcat, _)

neighbour_deltas = [
  CartesianIndex(-1, 0), CartesianIndex(1, 0), CartesianIndex(0, -1), CartesianIndex(0, 1),
  CartesianIndex(-1, -1), CartesianIndex(-1, 1), CartesianIndex(1, -1), CartesianIndex(1, 1)
]

issymbol(ch) = !(isdigit(ch) || ch == '.')

mutable struct State
  digits::Vector{Char}
  positions::Vector{CartesianIndex{2}}

  State() = new([], [])
end

function add_digit!(s::State, digit::Char, pos::CartesianIndex{2})
  push!(s.digits, digit)
  push!(s.positions, pos)
end

function push_if_part_number!(part_numbers::Vector{Int64}, state::State, symbol_positions::Set{CartesianIndex{2}})
  candidate = @pipe String(state.digits) |> parse(Int64, _)
  all_neighbour_positions = map(pos -> [pos] .+ neighbour_deltas, state.positions) |> Iterators.flatten
  if !(intersect(symbol_positions, all_neighbour_positions) |> isempty)
    push!(part_numbers, candidate)
  end
end

function part_numbers(inp)
  part_numbers = Vector{Int64}()
  symbol_positions = findall(issymbol, inp) |> Set
  # sort them in a row-by-row order, so we can rely on digits of one number following each other
  digit_positions = @pipe findall(isdigit, inp) |> sort(_, by=x -> (x[1], x[2]))

  state = State()
  for digit_position in digit_positions
    # if we have digits and the current digit doesn't directly follow them, we can push the number and clear the state
    if !isempty(state.digits) && (digit_position != last(state.positions) + CartesianIndex(0, 1))
      push_if_part_number!(part_numbers, state, symbol_positions)
      state = State()
    end

    add_digit!(state, inp[digit_position], digit_position)
  end

  if !isempty(state.digits)
    push_if_part_number!(part_numbers, state, symbol_positions)
  end

  part_numbers
end

p1(inp) = part_numbers(inp) |> sum

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