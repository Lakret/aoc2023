using Pipe

parse_input(input) = @pipe strip(input) |> split(_, "\n") |> split.(_) |> map(x -> parse.(Int64, x), _)

function predict(value_history; forward=true)
  sequence = deepcopy(value_history)
  sequences = [sequence]

  while !all(x -> x == 0, sequence)
    sequence = @pipe [sequence[i+1] - sequence[i] for i in eachindex(sequence) if i < length(sequence)]
    push!(sequences, sequence)
  end

  for i in (length(sequences)-1):-1:1
    seq = sequences[i]

    if forward
      push!(seq, last(seq) + last(sequences[i+1]))
    else
      pushfirst!(seq, first(seq) - first(sequences[i+1]))
    end
  end

  forward ? last(sequences[1]) : first(sequences[1])
end

test_history = parse_input("""
  0 3 6 9 12 15
  1 3 6 10 15 21
  10 13 16 21 30 45
  """)
history = readchomp("inputs/d09") |> parse_input

p1(history) = predict.(history) |> sum
p2(history) = predict.(history; forward=false) |> sum

@assert predict.(test_history) == [18, 28, 68]
@assert p1(test_history) == 114
@time @assert @show p1(history) == 1930746032

@assert predict.(test_history; forward=false) == [-3, 0, 5]
@assert p2(test_history) == 2
@time @assert @show p2(history) == 1154
