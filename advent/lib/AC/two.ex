defmodule AC.Two do

  @moduledoc """
  Solution to AOC 2
  """
  use Problem
  def test_input() do
"""
A Y
B X
C Z
"""
  end
  def test_output_part1 do
    15
  end
  def test_output_part2 do
    12
  end
  def problem1(input) do
    String.split(input,"\n")
    |> Enum.filter(fn x -> String.length(x)!=0 end)
    |> Enum.map(fn line -> String.split(line) end)
    |> Enum.map(fn pair -> parse_line(pair) end)
    |> Enum.map(fn play -> parse_line_p1(play) end)
    |> Enum.map(fn play -> process_play_p1(play) end)
    |> Enum.sum()
  end

  def problem2(input) do
    String.split(input,"\n")
    |> Enum.filter(fn x -> String.length(x)!=0 end)
    |> Enum.map(fn line -> String.split(line) end)
    |> Enum.map(fn pair -> parse_line(pair) end)
    |> Enum.map(fn play -> process_play_p2(play) end)
    |> Enum.sum()
  end

  defp parse_line(line) do
    %{opponent: parse_card_code(hd(line)), self: hd(tl(line))}
  end
  defp parse_card_code(char) do
    cond do
      char == "A" -> :rock
      char == "B" -> :paper
      char == "C" -> :scissors
      true -> raise "invalid card"
    end
  end
  defp parse_line_p1(play) do
    %{opponent: play[:opponent], self: parse_card_code_self(play[:self])}
  end
  defp parse_card_code_self(char) do
    cond do
      char == "X" -> :rock
      char == "Y" -> :paper
      char == "Z" -> :scissors
      true -> raise "invalid card"
    end
  end
    @doc """
  gets score for given play
  iex> process_play(%{opponent: "A", self: "Y"})
  8
  iex> process_play(%{opponent: "B", self: "X"})
  1
  iex> process_play(%{opponent: "C", self: "Z"})
  6
  """
  def process_play_p1(play) do

    get_card_score(play[:self]) + cond do
      AC.Two.CardTable.win_map()[play[:opponent]] == play[:self] -> 6
      AC.Two.CardTable.draw_map()[play[:opponent]] == play[:self] -> 3
      AC.Two.CardTable.lose_map()[play[:opponent]] == play[:self] -> 0
      true -> raise "invalid play"
    end
  end


  def get_card_score(card) do
    cond do
      card == :rock -> 1
      card == :paper -> 2
      card == :scissors -> 3
      true -> raise "invalid card score"
    end
  end
  defmodule CardTable do
    def win_map() do
      %{rock: :paper, paper: :scissors, scissors: :rock}
    end
    def draw_map() do
      %{rock: :rock, paper: :paper, scissors: :scissors}
    end
    def lose_map() do
      %{rock: :scissors, paper: :rock, scissors: :paper}
    end

  end
    @doc """
  gets score for given play according to new play system
  iex> process_play(%{opponent: :rock, self: "Y"})
  4
  iex> process_play(%{opponent: :paper, self: "X"})
  1
  iex> process_play(%{opponent: :scissors, self: "Z"})
  7
  """
  def process_play_p2(play) do
    cond do
      play[:self] == "X" -> 0 + get_card_score(CardTable.lose_map()[play[:opponent]])
      play[:self] == "Y" -> 3 + get_card_score(CardTable.draw_map()[play[:opponent]])
      play[:self] == "Z" -> 6 + get_card_score(CardTable.win_map()[play[:opponent]])
      true -> raise "invalid player hand"
    end
  end

end
