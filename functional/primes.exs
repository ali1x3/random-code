defmodule Primes do
  def checkPrime(num) do
    factors = checkPrime(1, num)
    #IO.puts(factors)
    if factors == 2 do
      :ok
    else
      :not
    end
  end

  def checkPrime(mod, num) when mod <= num do
    reminder = rem(num, mod)
    #IO.puts("reminder: #{reminder}")
    #IO.puts("mod: #{mod}")
    if(reminder == 0) do
      1 + checkPrime(mod + 1, num)
    else
      0 + checkPrime(mod + 1, num)
    end
  end

  def checkPrime(_mod, _num) do
    0
  end

  def nthPrime(nth) do
    nthPrime(nth, 0)
  end

  def nthPrime(1, currentnum) do
    currentnum
  end

  def nthPrime(nth, currentnum) do
    nextnum = getPrime(currentnum)
    nthPrime(nth - 1, nextnum)
  end

  def getPrime(num) do
    if checkPrime(num + 1) === :ok do
      num + 1
    else
      getPrime(num + 1)
    end
  end

end

nthprime = Primes.nthPrime(12)
IO.puts(nthprime)
