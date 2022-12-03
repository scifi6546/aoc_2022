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
    70
  end

  def problem1(input) do
    String.split(input, "\n")
    |> Enum.filter(fn x -> String.length(x) != 0 end)
    |> Enum.map(fn x -> split_line(x) end)
    |> Enum.map(fn line_dict -> find_repeat(line_dict[:left], line_dict[:right]) end)
    |> Enum.map(fn x -> hd(String.to_charlist(x)) end)
    |> Enum.map(fn x -> get_char_value(x) end)
    |> Enum.sum()
  end

  def problem2(input) do
    String.split(input, "\n")
    |> Enum.filter(fn x -> String.length(x) != 0 end)
    |> Enum.chunk_every(3)
    |> Enum.map(fn bags -> find_repeat_many(bags) end)
    |> Enum.map(fn chars -> get_graphene_value(chars) end)
    |> Enum.sum()
  end

  @doc """
  gets value of graphine
  """
  def get_graphene_value(s) do
    get_char_value(hd(String.to_charlist(s)))
  end

  @doc """
  gets value of char
  """
  def get_char_value(char) do
    cond do
      # caps
      char >= 0x41 && char <= 0x5A -> char - 0x41 + 27
      # lower
      char >= 0x61 && char <= 0x7A -> char - 0x61 + 1
      true -> raise "invalid char"
    end
  end

  @doc """
  Finds repeating chars in charlist
  """
  @spec find_repeat_all(String, String) :: [String]
  def find_repeat_all(left, right) do
    right = String.to_charlist(right)

    String.to_charlist(left)
    |> Enum.map(fn left_char ->
      Enum.map(
        right,
        fn right_char ->
          cond do
            right_char == left_char -> left_char
            true -> :null
          end
        end
      )
      |> Enum.filter(fn x -> x != :null end)
      |> Enum.map(fn x -> List.to_string([x]) end)
    end)
    |> Enum.filter(fn l -> l != [] end)
    |> Enum.map(fn l -> hd(l) end)
  end

  @doc """
  Converts list of strings to single string
  """
  @spec list_to_string([String]) :: String
  def list_to_string(l) do
    Enum.reduce(l, "", fn x, acc -> acc <> x end)
  end

  @doc """
  Finds repeating characters in list of strings, length of list must be at least 2
  """
  @spec find_repeat_many([String]) :: [String]
  def find_repeat_many(strings) do
    first = hd(strings)
    # next = tl(strings)
    # Enum.zip(strings,next)
    # |> Enum.map(fn {l,r} -> find_repeat_list(l,r)end)
    Enum.reduce(strings, hd(strings), fn x, acc -> list_to_string(find_repeat_all(x, acc)) end)
  end

  def two_iter(l) do
    two = tl(l)
    Enum.zip(l, two)
  end

  @doc """
  Finds first repeating char in charlist
  """
  @spec find_repeat(String, String) :: String
  def find_repeat(left, right) do
    find_repeat_all(left, right)
    |> hd()
  end

  @doc """
  Splits line in half
  """
  def split_line(line) do
    length = String.length(line)
    len_2 = Kernel.round(length / 2)

    %{
      left: String.slice(line, 0, len_2),
      right: String.slice(line, len_2, length - 1)
    }
  end
end
