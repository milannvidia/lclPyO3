import lclRust

eval=lclRust.Evaluation.tsp_from_dist_matrix("C:/Users/milan/Documents/Masterproef/lclPy/data/distanceMatrix")
move=lclRust.MoveType.swap(48)
problem=lclRust.Problem.array_problem(move,eval)
term=lclRust.Termination.max_iterations(1000)
cooling=lclRust.Cooling.geometric_cooling(0.95)
iter=lclRust.IterationsPerTemp.cnst_iter_temp(1000)

lcl=lclRust.LocalSearch.simulated_annealing(2000,True,problem,term,cooling,iter)
res=lcl.run()