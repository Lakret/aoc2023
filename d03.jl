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

function push_if_part_number!(
  part_numbers::Vector{Tuple{Int64,Set{CartesianIndex}}},
  state::State,
  symbol_positions::Set{CartesianIndex{2}}
)
  candidate = @pipe String(state.digits) |> parse(Int64, _)
  all_neighbour_positions = map(pos -> [pos] .+ neighbour_deltas, state.positions) |> Iterators.flatten
  neighbouring_symbol_positions = intersect(symbol_positions, all_neighbour_positions)
  if !isempty(neighbouring_symbol_positions)
    push!(part_numbers, (candidate, neighbouring_symbol_positions))
  end
end

function part_numbers(inp)
  part_numbers = Vector{Tuple{Int64,Set{CartesianIndex}}}()
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

p1(inp) = part_numbers(inp) .|> first |> sum

function gear_numbers(inp)
  gear_symbol_positions = findall(ch -> ch == '*', inp)

  candidate_gears = Dict{CartesianIndex{2},Vector{Int64}}()
  for (part_number, adjacent_symbol_positions) = part_numbers(inp)
    adjacent_gear_symbol_positions = intersect(adjacent_symbol_positions, gear_symbol_positions)

    for candidate_gear_position = adjacent_gear_symbol_positions
      pns = get(candidate_gears, candidate_gear_position, [])
      push!(pns, part_number)

      candidate_gears[candidate_gear_position] = pns
    end
  end

  candidate_gears = values(candidate_gears) |> collect
  filter(numbers -> length(numbers) == 2, candidate_gears)
end

p2(inp) = gear_numbers(inp) .|> prod |> sum

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

@assert p2(test_inp) == 467835
@assert @show p2(input) == 87605697
