defmodule ProblemReq do
  @doc """
  input that AOC gives you
  """
  @callback test_input() :: String
  @doc """
  input that AOC gives you for part 1
  """
  @callback test_output_part1() :: String

  @doc """
  input that AOC gives you for part 2
  """
  @callback test_output_part2() :: String
  @callback problem1(input :: String) :: any
  @callback problem2(input :: String) :: any
end

defmodule Problem do
  defmacro __using__(_opts) do
    # <--
    quote do
      # <--
      import Problem
      @behaviour ProblemReq
      def problem1() do
        problem1(test_input())
      end

      def problem1_timed(input) do
        Problem.timed_fun(input, fn input -> problem1(input) end)
      end

      def problem2() do
        problem2(test_input())
      end

      def problem2_timed(input) do
        Problem.timed_fun(input, fn input -> problem2(input) end)
      end
    end

    # <--
  end

  def timed_fun(input, fun) do
    t_start = Time.utc_now()
    output = fun.(input)
    t_end = Time.utc_now()
    %{output: output, time_ms: Time.diff(t_end, t_start, :microsecond) / 1000.0}
  end
end

defmodule AC.TestProblem do
  use Problem
  @spec test_input() :: String
  def test_input() do
    "Hi there"
  end

  def test_output_part1 do
    :good
  end

  def test_output_part2 do
    :better
  end

  def problem1(_input) do
    :good
  end

  def problem2(_input) do
    :better
  end
end

defmodule AC do
  @moduledoc """
  Documentation for `AC`.
  """
  @doc """
  Loads a file into a string
  """
  @spec load_file(String) :: String
  def load_file(path) do
    {:ok, file_content} = File.read(path)
    file_content
  end
end

defmodule GeneralHelpers do
end
