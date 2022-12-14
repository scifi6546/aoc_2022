defmodule AC.Seven do
  use Problem
  @spec test_input() :: String
  def test_input() do
    """
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    """
  end

  def test_output_part1 do
    95437
  end

  @spec test_output_part2 :: :better
  def test_output_part2 do
    24_933_642
  end

  def problem1(input) do
    String.split(input, "$")
    |> Enum.filter(fn s -> String.length(s) != 0 end)
    |> Enum.map(fn s -> handle_command(s) end)
    |> run_commands()
    |> Map.get(:files)
    |> get_directory_sizes()
    |> list_directory_sizes()
    |> Enum.map(fn {size, _name} -> size end)
    |> Enum.filter(fn size -> size <= 100_000 end)
    |> Enum.sum()
  end

  def problem2(input) do
    files =
      String.split(input, "$")
      |> Enum.filter(fn s -> String.length(s) != 0 end)
      |> Enum.map(fn s -> handle_command(s) end)
      |> run_commands()
      |> Map.get(:files)
      |> get_directory_sizes()

    directory_sizes = list_directory_sizes(files)

    size = files[:size]
    available_space = 70_000_000
    needed_space = 30_000_000

    extra_space = needed_space - (available_space - size)

    Enum.map(directory_sizes, fn {size, _name} -> size end)
    |> Enum.filter(fn size -> size >= extra_space end)
    |> Enum.min()
  end

  def get_root_size(file_tree) do
    Enum.map(file_tree, fn {name, val} -> {name, val} end)
  end

  def list_directory_sizes(file_tree) do
    Enum.flat_map(file_tree[:files], fn {file_name, file} ->
      if file[:type] == :directory do
        [{file[:size], file_name}] ++ list_directory_sizes(file)
      else
        []
      end
    end)
  end

  def get_directory_sizes(file_tree) do
    if Map.has_key?(file_tree, :files) do
      new_map = Map.update(file_tree, :size, 0, fn _size -> 0 end)

      new_tree =
        Map.update!(new_map, :files, fn existing_files ->
          Enum.reduce(Map.keys(existing_files), %{}, fn key, acc ->
            get_directory_sizes(existing_files[key])
            Map.put(acc, key, get_directory_sizes(existing_files[key]))
          end)
        end)

      size =
        Enum.map(Map.keys(new_tree[:files]), fn file_key ->
          new_tree[:files][file_key][:size]
        end)
        |> Enum.sum()

      Map.put(new_tree, :size, size)
    else
      file_tree
    end
  end

  defp run_commands(commands) do
    initial_tree = %{current_directory: [], files: %{}}

    if hd(commands)[:command] != "cd" do
      raise "must first do cd, todo figure out how to fix if that doesnt happen"
    end

    Enum.reduce(tl(commands), initial_tree, fn command, acc ->
      cond do
        command[:command] == "cd" ->
          run_cd(acc, command)

        command[:command] == "ls" ->
          run_ls(acc, command)

        true ->
          raise "invalid command"
      end
    end)
  end

  defp run_cd(file_tree, command) do
    Map.update(file_tree, :current_directory, [], fn initial_value ->
      cond do
        command[:data] == ".." ->
          Enum.take(initial_value, Enum.count(initial_value) - 1)

        true ->
          initial_value ++ [command[:data]]
      end
    end)
  end

  defp run_ls(file_tree, command) do
    Map.update!(file_tree, :files, fn files ->
      Enum.reduce(command[:data], files, fn add_file, file_acc ->
        update_file_tree(file_acc, file_tree[:current_directory], add_file)
      end)
    end)
  end

  defp update_file_tree(file_tree, parent_path, file) do
    cond do
      Enum.count(parent_path) > 0 ->
        Map.update!(file_tree, :files, fn dir_map ->
          Map.update!(dir_map, hd(parent_path), fn dir ->
            update_file_tree(dir, tl(parent_path), file)
          end)
        end)

      Enum.count(parent_path) == 0 ->
        Map.update(file_tree, :files, place_files(%{}, file), fn existing_tree ->
          place_files(existing_tree, file)
        end)

      true ->
        raise "invalid state"
    end
  end

  defp place_files(file_tree, file) do
    cond do
      file[:type] == :directory ->
        Map.put(file_tree, file[:name], %{type: :directory})

      file[:type] == :file ->
        Map.put(file_tree, file[:name], %{type: :file, size: file[:size]})

      true ->
        raise "invalid file type"
    end
  end

  defp handle_command(s) do
    contains_cd =
      String.trim(s)
      |> String.starts_with?("cd")

    contains_ls =
      String.trim(s)
      |> String.starts_with?("ls")

    cond do
      contains_cd -> %{command: "cd", data: handle_cd(s)}
      contains_ls -> %{command: "ls", data: handle_ls(s)}
      true -> raise "invalid command"
    end
  end

  defp handle_cd(str) do
    String.trim(str)
    |> String.split()
    |> tl()
    |> hd()
  end

  defp handle_ls(str) do
    String.trim(str)
    |> String.split("\n")
    |> tl()
    |> Enum.map(fn row ->
      line = String.split(row)
      {type, size} = get_type(hd(line))
      %{type: type, size: size, name: hd(tl(line))}
    end)
  end

  defp get_type(str) do
    if str == "dir" do
      {:directory, 0}
    else
      {:file, String.to_integer(str)}
    end
  end
end
