defmodule Day2 do
  @enforce_keys [:id]
  defstruct [:id, :valid, :red, :green, :blue]
  
  defp get_max_rgb(scores, rgb) when length(scores) == 0 do
    rgb
  end

  defp get_max_rgb(scores, rgb) do
    [score, color] = Enum.take(scores, 2)
    scores = Enum.drop(scores, 2)
    idx = case color do
      "red" -> 0
      "green" -> 1
      "blue" -> 2 
    end

    {score, _ } = Integer.parse(score)
    rgb = put_elem(rgb, idx, max(score, elem(rgb, idx)))
    get_max_rgb(scores, rgb)
  end

  defp get_rgb(game) do
    String.split(hd(game), ";") |> 
      Enum.map(fn s -> String.split(s, ", ") end) |>
      Enum.map(fn s -> Enum.map(s, fn x -> String.split(String.trim(x), " ") end) end) |>
      List.flatten() |>
      get_max_rgb({0, 0, 0})
  end

  defp get_game(head) do
    [game | scores] = String.split(head, ": ")
    [_, id] = String.split(game, " ")
    {id, _} = Integer.parse(id)
    {r, g, b} = get_rgb(scores)

    max_r = 12
    max_g = 13
    max_b = 14
    valid = 
      r <= max_r and
      g <= max_g and
      b <= max_b

    %Day2{id: id, valid: valid, red: r, green: g, blue: b}
  end

  def part1(input, games) when length(input) == 1 do
    head = hd(input)
    games = games ++ [get_game(head)]
    sum = Enum.filter(games, fn g -> g.valid end) |>
      Enum.reduce(0, fn g, acc -> g.id + acc end)
    IO.inspect(sum)
  end
  
  def part1(input, games) do
    [head | tail] = input
    games = games ++ [get_game(head)]
    part1(tail, games)
  end

  def part2(input, games) when length(input) == 1 do
    head = hd(input)
    games = games ++ [get_game(head)]
    sum = Enum.reduce(games, 0, fn g, acc -> (g.red * g.green * g.blue) + acc end)
    IO.inspect(sum)
  end
  
  def part2(input, games) do
    [head | tail] = input
    games = games ++ [get_game(head)]
    part2(tail, games)
  end
end


file = File.read!("./input.txt")
file = String.trim(file)
file = String.split(file, "\n")

Day2.part1(file, [])
Day2.part2(file, [])
