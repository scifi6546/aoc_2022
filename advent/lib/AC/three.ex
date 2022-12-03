defmodule AC.Three do
  use Problem
  def test_input() do
"""
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"""
  end
  def test_output_part1 do
    157
  end
  def test_output_part2 do
    :better
  end
  def problem1(input) do
    String.split(input,"\n")
    |> Enum.filter(fn x -> String.length(x)!=0 end)
    |> Enum.map(fn x -> split_line(x) end)
    |> Enum.map(fn x -> find_repeat(x) end)
    |> Enum.map(fn x -> hd(String.to_charlist(x))end)
    |> Enum.map(fn x -> get_string_value(x) end)
    |> Enum.sum()
  end

  def problem2(_input) do
    :better
  end
  @doc """
  gets value of char
  """
  def get_string_value(char) do
    cond do
      # caps
      char >= 0x41 && char <= 0x5a -> char-0x41 + 27
      # lower
      char >= 0x61 && char <= 0x7a -> char - 0x61+1
      true -> raise "invalid char"
    end
  end
  def find_repeat(input) do
    right = String.to_charlist(input[:right])
    String.to_charlist(input[:left])
    |> Enum.map(
      fn left_char ->
        Enum.map(right,
          fn right_char ->
            cond do
              right_char == left_char -> left_char
              true -> :null
            end
          end)
        |> Enum.filter(fn x -> x != :null end)
        |> Enum.map(fn x -> List.to_string([x]) end)
        #|> Enum.map(fn x -> String.first(x) end)
      end
    )
    |> Enum.filter(fn l -> l != [] end)
    |> Enum.map(fn l -> hd(l) end)
    |> hd()




  end
  @doc """
  Splits line in half
  """
  def split_line(line) do
    length = String.length(line)
    len_2 = Kernel.round(length/2)
    %{
      left: String.slice(line,0,len_2),
      right: String.slice(line,len_2,length-1)
    }
  end
end
