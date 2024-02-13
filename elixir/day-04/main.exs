defmodule Day4 do

  defp get_winning_and_cards(line) do
    [_num, cards] = String.split(line, ": ")
    String.split(cards, " | ")
  end

  defp get_card_numbers(cards) do
    String.split(cards, " ") |> Enum.filter(fn s -> s !== "" end)
  end

  defp get_score_from_count(count) do
    case count do
      0 -> 0
      _ -> Integer.pow(2, count - 1)
    end
  end

  defp update_card_counts(cards, amount, first, last) do
    cond do
      first === last -> cards
      true -> 
        cards = List.update_at(cards, first, &(&1 + amount))
        update_card_counts(cards, amount, first + 1, last)
    end 
  end

  defp get_card_counts(counts, cards, _current) when length(counts) === 1 do
    cards
  end
  
  defp get_card_counts(counts, cards, current) do
    [head | tail] = counts
    amount = Enum.at(cards, current - 1)
    cards = update_card_counts(cards, amount, current, current + head)
    get_card_counts(tail, cards, current + 1)
  end

  defp get_count(line) do
    [winning, cards] = get_winning_and_cards(line)
    winning = get_card_numbers(winning)
    cards = get_card_numbers(cards)
    Enum.count(cards, fn c -> Enum.member?(winning, c) end)
  end

  defp get_score(line) do
    count = get_count(line)
    get_score_from_count(count)
  end

  def part1(input) do
    score = Enum.map(input, fn s -> get_score(s) end) |> Enum.sum()
    IO.inspect(score)
  end

  def part2(input) do
    matching_counts = Enum.map(input, fn s -> get_count(s) end)
    total_counts = List.duplicate(1, length(input))
    final = get_card_counts(matching_counts, total_counts, 1)
    sum = Enum.sum(final)
    IO.inspect(sum)
  end
end

file = File.read!("./input.txt")
file = String.trim(file)
file = String.split(file, "\n")

Day4.part1(file)
Day4.part2(file)
