defmodule Armstrong do
  def countNums(num) when num < 10 do
    1
  end

  def countNums(num) do
    newnum = div(num, 10)
    1 + countNums(newnum)
  end

  def armstrong(num) do
    digitnums = countNums(num)
    armstrong(num, digitnums)
  end

  def armstrong(0, _digitnums) do
    0
  end

  def armstrong(num, digitnums) when num > 0 do
    digit = rem(num, 10)
    newnum = div(num, 10)
    digit**digitnums + armstrong(newnum, digitnums)
  end

  def checkarmstrong(num) do
    if num == armstrong(num) do
      :ok
    else
      :no
    end
  end
end 

{num, _} = IO.gets("Input number to check if armstrong > " ) |> Integer.parse();
IO.puts(Armstrong.countNums(num))
IO.puts(Armstrong.checkarmstrong(num))
