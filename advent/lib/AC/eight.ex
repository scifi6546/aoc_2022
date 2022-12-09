defmodule AC.Eight do
  use Problem

  defmodule Grid do
    @spec zip([Any], Any) :: Any
    def zip(grids, map_fn) do
      Enum.map(grids, fn grid -> Tuple.to_list(grid) end)
      |> Enum.zip()
      |> Enum.map(fn row ->
        Tuple.to_list(row)
        |> Enum.map(fn row -> Tuple.to_list(row) end)
        |> Enum.zip()
        |> Enum.map(fn item -> map_fn.(item) end)
        |> List.to_tuple()
      end)
      |> List.to_tuple()
    end

    def max(grid) do
      Tuple.to_list(grid)
      |> Enum.map(fn row -> Tuple.to_list(row) |> Enum.max() end)
      |> Enum.max()
    end

    def to_string(grid) do
      Tuple.to_list(grid)
      |> Enum.map(fn row ->
        Tuple.to_list(row)
        |> Enum.map(fn x -> Integer.to_string(x) end)
        |> Enum.reduce("", fn x, acc -> acc <> " " <> x end)
      end)
      |> Enum.reduce(fn x, acc -> acc <> "\n" <> x end)
    end
  end

  @spec test_input() :: String
  def test_input() do
    """
    30373
    25512
    65332
    33549
    35390
    """
  end

  def test_output_part1 do
    21
  end

  def test_output_part2 do
    :better
  end

  def problem1(input) do
    grid =
      String.split(input, "\n")
      |> Enum.filter(fn s -> String.length(s) != 0 end)
      |> Enum.map(fn s -> String.graphemes(s) end)
      |> Enum.map(fn row -> list_to_ints(row) end)
      |> Enum.map(fn row -> List.to_tuple(row) end)
      |> List.to_tuple()

    Enum.zip([
      get_u_d_vis_grid(grid),
      get_d_u_vis_grid(grid),
      get_l_r_vis_grid(grid),
      get_r_l_vis_grid(grid)
    ])
    |> Enum.map(fn rows -> compare_rows(rows) end)
    |> Enum.map(fn row ->
      Enum.map(row, fn item ->
        if item == :visible do
          1
        else
          0
        end
      end)
    end)
    |> Enum.map(fn row -> Enum.sum(row) end)
    |> Enum.sum()
  end

  def problem2(input) do
    grid = grid_from_string(input)

    l_r =
      grid_iter_rows(grid)
      |> Enum.map(fn row -> get_row_left_dist(row) end)
      |> grid_from_rows()

    r_l =
      grid_iter_rows(grid)
      |> Enum.map(fn row -> get_row_right_dist(row) end)
      |> grid_from_rows()

    u_d = get_u_dist(grid)
    d_u = get_d_dist(grid)

    Grid.zip([l_r, r_l, u_d, d_u], fn x_tuple -> Tuple.to_list(x_tuple) |> Enum.product() end)
    # |> Grid.max()
    # {l_r, r_l, u_d, d_u}
  end

  def grid_from_string(input) do
    String.split(input, "\n")
    |> Enum.filter(fn s -> String.length(s) != 0 end)
    |> Enum.map(fn s -> String.graphemes(s) end)
    |> Enum.map(fn row -> list_to_ints(row) end)
    |> Enum.map(fn row -> List.to_tuple(row) end)
    |> List.to_tuple()
  end

  def get_u_dist(grid) do
    grid_iter_cols(grid)
    |> Enum.map(fn col -> get_row_left_dist(col) end)
    |> cols_to_grid()
  end

  def get_d_dist(grid) do
    grid_iter_cols(grid)
    |> Enum.map(fn col -> get_row_right_dist(col) end)
    |> cols_to_grid()
  end

  def grid_iter_rows(grid) do
    Tuple.to_list(grid)
  end

  def grid_iter_cols(grid) do
    x_size = get_x_size(grid)
    Enum.map(AC.range(0, x_size), fn x -> get_grid_col(grid, x) end)
  end

  def cols_to_grid(cols) do
    grid = List.to_tuple(cols)
    Enum.map(0..(get_x_size(grid) - 1), fn x -> get_grid_col(grid, x) end) |> List.to_tuple()
  end

  def grid_from_rows(grid) do
    List.to_tuple(grid)
  end

  def get_row_right_dist(row) do
    Tuple.to_list(row)
    |> Enum.reverse()
    |> List.to_tuple()
    |> get_row_left_dist()
    |> Tuple.to_list()
    |> Enum.reverse()
    |> List.to_tuple()
  end

  def get_row_left_dist(row) do
    Enum.map(0..(tuple_size(row) - 1), fn i ->
      Enum.map(AC.range(0, i), fn j -> %{i: i, j: j, elem: elem(row, j)} end)
      |> Enum.reverse()
      |> Enum.reduce(%{largest: elem(row, i), current_distance: i, in_shadow: false}, fn x, acc ->
        cond do
          acc[:in_shadow] == false && x[:elem] >= acc[:largest] ->
            Map.put(acc, :largest, x[:elem])
            |> Map.put(:current_distance, i - x[:j])
            |> Map.put(:in_shadow, true)

          acc[:in_shadow] == true && x[:elem] > acc[:largest] ->
            Map.put(acc, :largest, x[:elem])
            |> Map.put(:current_distance, i - x[:j])

          true ->
            acc
        end
      end)
      |> Map.get(:current_distance)

      # |> Map.get(:current_distance)
    end)
    |> List.to_tuple()
  end

  def compare_rows(rows) do
    row_length = tuple_size(elem(rows, 0))

    Enum.map(0..(row_length - 1), fn x ->
      Enum.map(0..(tuple_size(rows) - 1), fn y -> elem(elem(rows, y), x) end)
      |> Enum.reduce(:hidden, fn x, acc ->
        if acc == :visible do
          acc
        else
          x
        end
      end)
    end)

    # Tuple.to_list(rows)
    # |> Enum.map(fn grid_row -> Tuple.to_list(grid_row) end)
  end

  defp is_visible(atom) do
    IO.inspect(atom)

    if atom == :visible do
      true
    else
      false
    end
  end

  def get_u_d_vis_grid(grid) do
    new_grid =
      Enum.map(0..(get_x_size(grid) - 1), fn x -> get_grid_col(grid, x) end)
      |> Enum.map(fn row -> row_visible_l_to_r(row) end)
      |> List.to_tuple()

    # flipping grid back
    Enum.map(0..(get_x_size(grid) - 1), fn x -> get_grid_col(new_grid, x) end)
  end

  def get_d_u_vis_grid(grid) do
    new_grid =
      Enum.map(0..(get_x_size(grid) - 1), fn x -> get_grid_col(grid, x) end)
      |> Enum.map(fn row -> row_visible_r_to_l(row) end)
      |> List.to_tuple()

    # flipping grid back
    Enum.map(0..(get_x_size(grid) - 1), fn x -> get_grid_col(new_grid, x) end)
  end

  def get_l_r_vis_grid(grid) do
    Enum.map(0..(get_y_size(grid) - 1), fn y -> get_grid_row(grid, y) end)
    |> Enum.map(fn row -> row_visible_l_to_r(row) end)
  end

  def get_r_l_vis_grid(grid) do
    Enum.map(0..(get_y_size(grid) - 1), fn y -> get_grid_row(grid, y) end)
    |> Enum.map(fn row -> row_visible_r_to_l(row) end)
  end

  @doc """
  gets which elements are visible left to right
  """
  def row_visible_l_to_r(row) do
    Tuple.to_list(row)
    |> Enum.map_reduce(-1, fn x, acc ->
      {if x > acc do
         :visible
       else
         :hidden
       end, max(acc, x)}
    end)
    |> elem(0)
    |> List.to_tuple()
  end

  @doc """
  gets which elements are visible left to right
  """
  def row_visible_r_to_l(row) do
    Tuple.to_list(row)
    |> Enum.reverse()
    |> List.to_tuple()
    |> row_visible_l_to_r()
    |> Tuple.to_list()
    |> Enum.reverse()
    |> List.to_tuple()
  end

  def get_grid_coord(grid, x, y) do
    elem(elem(grid, y), x)
  end

  def get_grid_row(grid, y) do
    elem(grid, y)
  end

  def get_y_size(grid) do
    tuple_size(grid)
  end

  def get_x_size(grid) do
    tuple_size(elem(grid, 0))
  end

  def get_grid_col(grid, x) do
    Enum.map(0..(get_y_size(grid) - 1), fn i ->
      elem(
        elem(grid, i),
        x
      )
    end)
    |> List.to_tuple()
  end

  defp list_to_ints(char_list) do
    Enum.map(char_list, fn char -> String.to_integer(char) end)
  end
end
