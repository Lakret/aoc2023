
using Pipe
parse_input(input) = @pipe input |> strip |> split(_, "\n") |> collect.(_) |> mapreduce(permutedims, vcat, _)

find_start_coords(grid) = findfirst(x -> x == 'S', grid)

NEIGHBORS = [CartesianIndex(-1, 0), CartesianIndex(1, 0), CartesianIndex(0, -1), CartesianIndex(0, 1)]

function allowed_neighbors(grid, pos)::Set{CartesianIndex{2}}
  candidates = [pos] .+ NEIGHBORS
  filter(
    x -> x[1] > 0 && x[2] > 0 && x[1] <= size(grid)[1] && x[2] <= size(grid)[2] && grid[x] != '#',
    candidates
  ) |> Set
end

function move(grid, position::CartesianIndex{2}, max_steps)::Set{CartesianIndex{2}}
  move(grid, Set([position]), max_steps)
end

function move(
  grid,
  positions::Set{CartesianIndex{2}},
  max_steps
)::Set{CartesianIndex{2}}
  if max_steps == 0
    positions
  else
    new_pos = Set{CartesianIndex{2}}()

    for pos in positions
      new_pos = union(new_pos, allowed_neighbors(grid, pos))
    end

    move(grid, new_pos, max_steps - 1)
  end
end

p1(grid, max_steps=64) = move(grid, find_start_coords(grid), max_steps) |> length

test_grid = parse_input("""
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
""")

grid = readchomp("inputs/d21") |> parse_input

@assert p1(test_grid, 6) == 16
@time @assert @show p1(grid) == 3578