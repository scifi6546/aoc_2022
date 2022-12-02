defmodule AC do
  @moduledoc """
  Documentation for `AC`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> AC.hello()
      :world

  """
  def hello do
    :world
  end
  @doc """
  First advent of code, takes in path of test input
  iex> AC.one("./test/test_input/1")
  24000
  """
  def one(path) do
    AC.OneHelper.parse_file(path)
    |> Enum.reduce( 0,fn x,acc ->
      cond do
        x[:sum] > acc -> x[:sum]
        true -> acc
      end
    end)
  end
  def one_p2(path) do
    AC.OneHelper.parse_file(path)
      |> Enum.map(fn x -> x[:sum] end)
      |> Enum.sort(fn x,y -> x >= y end)
      |> Enum.take(3)
      |> Enum.sum()
  end
  def two_test_input() do
"""
A Y
B X
C Z
"""
  end
  def two_p1(input) do
    String.split(input,"\n")
    |> Enum.filter(fn x -> String.length(x)!=0 end)
    |> Enum.map(fn line -> String.split(line) end)
    |> Enum.map(fn pair -> AC.TwoHelper.parse_line(pair) end)
    |> Enum.map(fn play -> AC.TwoHelper.process_play_p1(play) end)
    |> Enum.sum()
  end
  def two_p1() do
    two_p1(two_test_input())
  end

  def two_p2(input) do
    String.split(input,"\n")
    |> Enum.filter(fn x -> String.length(x)!=0 end)
    |> Enum.map(fn line -> String.split(line) end)
    |> Enum.map(fn pair -> AC.TwoHelper.parse_line(pair) end)
    |> Enum.map(fn play -> AC.TwoHelper.process_play_p2(play) end)
    |> Enum.sum()
  end
  def two_p2() do
    two_p2(two_test_input())
  end
end
defmodule AC.TwoHelper do
  def parse_line(line) do
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
  defp get_card_value(card) do
    cond do
      card == :rock -> 1
      card == :paper -> 2
      card == :scissors -> 3
      true -> raise "invalid opponent hand"
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
      play[:self] == "X" -> 0 + get_card_score(get_lose_hand(play))
      play[:self] == "Y" -> 3 + get_card_score(get_draw_hand(play))
      play[:self] == "Z" -> 6 + get_card_score(get_win_hand(play))
      true -> raise "invalid player hand"
    end
  end
  defp get_lose_hand(play) do
    cond do
      play[:opponent] == :rock -> :scissors
      play[:opponent] == :paper -> :rock
      play[:opponent] == :scissors -> :paper
      true -> raise "invalid opponent hand"
    end
  end
  defp get_draw_hand(play) do
    play[:opponent]
  end
  defp get_win_hand(play) do
    cond do
      play[:opponent] == :rock -> :paper
      play[:opponent] == :paper -> :scissors
      play[:opponent] == :scissors -> :rock
      true -> raise "invalid opponent hand"
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
    cond do
      play[:opponent] == :rock -> opp_a(play)
      play[:opponent] == :paper -> opp_b(play)
      play[:opponent] == :scissors -> opp_c(play)
      true -> raise "invalid opponent move"
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
  @doc """
  Gets score if opponent played a
  """
  defp opp_a(play) do
    cond do
      play[:self] == "X" -> 1 + 3
      play[:self] == "Y" -> 2 + 6
      play[:self] == "Z" -> 3 + 0
      true -> raise "invalid player move"
    end
  end
  defp opp_b(play) do
    cond do
      play[:self] == "X" -> 1 + 0
      play[:self] == "Y" -> 2 + 3
      play[:self] == "Z" -> 3 + 6
      true -> raise "invalid player move"
    end
  end
  defp opp_c(play) do
    cond do
      play[:self] == "X" -> 1 + 6
      play[:self] == "Y" -> 2 + 0
      play[:self] == "Z" -> 3 + 3
      true -> raise "invalid player move"
    end
  end
end
defmodule AC.OneHelper do
  def parse_file(path) do
    GeneralHelpers.load_file(path)
    |> parse_to_struct()
  end

  def parse_to_struct(str) do
    String.split(str, "\n")
    |> Enum.chunk_by(fn(x) -> x=="" end)
    |> Enum.filter(fn x -> x != [""] end)
    |> Enum.map(
      fn arr -> AC.OneHelper.parse_int_list(arr)
      end)
    |> Enum.map(fn arr -> AC.OneHelper.to_sum_struct(arr)end)
  end
  def parse_int(s) do
    {num,_} = Integer.parse(s)
    num
  end
  def parse_int_list(l) do
    Enum.map(l,fn x ->
      parse_int(x)
    end)
  end
  def to_sum_struct(foods) do
    %{sum: Enum.sum(foods),elements: foods}
  end
  defmodule FoodSum do
    defstruct sum: 0, elements: []
  end
end
defmodule GeneralHelpers do
  def load_file(path) do
    {:ok,file_content} = File.read(path)
    file_content
  end
end
