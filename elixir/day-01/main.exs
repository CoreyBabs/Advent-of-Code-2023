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

  defp append_indices(line, indices, word, digit) do
    words = scan_and_map_indices(line, word, digit)
    indices = indices ++ words

    nums = scan_and_map_indices(line, digit, digit)
    indices ++ nums
  end

  defp scan_and_map_indices(line, src, dst) do
    {_, r} = Regex.compile(src)
    list = Regex.scan(r, line, return: :index)
    Enum.map(list, fn t -> {elem(List.first(t), 0), dst} end)
  end

  defp get_indices_of_numbers(line, indices, digits, total_digits) when length(digits) === 1 do
    head = hd(digits)
    v = total_digits[head]

    indices = append_indices(line, indices, head, v)
    
    Enum.sort_by(indices, fn {value, _} -> value end)
  end

  defp get_indices_of_numbers(line, indices, digits, total_digits) do
    [head | tail] = digits
    v = total_digits[head]

    indices = append_indices(line, indices, head, v)

    get_indices_of_numbers(line, indices, tail, total_digits)
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

  def part2(input, sum, total_digits) when length(input) > 1 do
    [head | tail] = input
    keys = Map.keys(total_digits)
    idxs = get_indices_of_numbers(head, [], keys, total_digits)
    first = List.first(idxs)
    last = List.last(idxs)
    tuple = {elem(first, 1), elem(last, 1)}

    {a, _} = combine_tuple(tuple)
    sum = sum + a
    part2(tail, sum, total_digits)
  end

  def part2(input, sum, total_digits) do
    head = hd(input)
    keys = Map.keys(total_digits)

    idxs = get_indices_of_numbers(head, [], keys, total_digits)
    first = List.first(idxs)
    last = List.last(idxs)
    tuple = {elem(first, 1), elem(last, 1)}

    {a, _} = combine_tuple(tuple)
    sum = sum + a
    IO.puts(sum)
  end
end

  digit_chars = %{
    "one" => "1",
    "two" => "2",
    "three" => "3",
    "four" => "4",
    "five" => "5",
    "six" => "6",
    "seven" => "7",
    "eight" => "8",
    "nine" => "9",
  }

file = File.read!("./input.txt")
file = String.trim(file)
file = String.split(file, "\n")

Day1.part1(file, 0)
Day1.part2(file, 0, digit_chars)
