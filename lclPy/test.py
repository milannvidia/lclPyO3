import lclRust
cooling=lclRust.Cooling.geometric_cooling(0.95)
iteration=lclRust.IterationsPerTemp.cnst_iter_temp(1000)
termination=lclRust.Termination.max_sec(5)
problem=lclRust.Problem.TSP([[0]])
lcl=lclRust.LocalSearch.simulated_annealing(problem,termination,cooling,iteration);
res=lcl.run()
print(res)