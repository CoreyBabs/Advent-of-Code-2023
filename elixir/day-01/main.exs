defmodule Day1 do
  defp get_digit(line, digits) when byte_size(line) === 0 do
    digits 
  end
  defp get_digit(line, digits) when tuple_size(digits) > 1 do
    head = String.first(line)
    case Integer.parse(head) do
      {_a, _} -> 
        digits = put_elem(digits, 1, head)
        get_digit(String.slice(line, 1..-1//1), digits)
      _ -> get_digit(String.slice(line, 1..-1//1), digits)
    end
  end 

  defp get_digit(line, digits) when tuple_size(digits) < 2 do
    head = String.first(line)
    case Integer.parse(head) do
      {_a, _} -> 
        digits = Tuple.append(digits, head)
        get_digit(String.slice(line, 1..-1//1), digits)
      _ -> get_digit(String.slice(line, 1..-1//1), digits)
    end
  end

  defp combine_tuple(pair) when tuple_size(pair) === 1 do
    combined = elem(pair, 0) <> elem(pair, 0)
    Integer.parse(combined)
  end

  defp combine_tuple(pair) when tuple_size(pair) === 2 do
    combined = elem(pair, 0) <> elem(pair, 1)
    Integer.parse(combined)
  end

  def part1(input, sum) when length(input) > 1 do
    [head | tail ] = input
    {a, _} = get_digit(head, {}) |> combine_tuple()
    sum = sum + a
    part1(tail, sum)
  end

  def part1(input, sum) do
    head = hd(input)
    {a, _} = get_digit(head, {}) |> combine_tuple()
    sum = sum + a
    IO.puts(sum)
  end
end

file = File.read!("./input.txt")
file = String.trim(file)
file = String.split(file, "\n")

Day1.part1(file, 0)
