from random import seed


def benchmark(algorithms, problems, stop_criterion=None, runs=10, seeds=None, log=False):
    """A function to perform multiple algorithms on multiple soltions.

    Note that the problems, algorithms and the stop criterion all need to have
    the method reset method properly implemented for this function to work
    properly. The logging of algorithms is determined by the logging parameter
    that was given to them at their construction. If it was True (default), the
    algorithm will log it's improvements and worse passed solutions. If logging
    is False, no logging will be shown for that particular algorithm.

    Parameters
    ----------
    problems : iterable object
        Contains all the problems.
    algorithms : iterable object
        Contains all the algorithms. Note that the algorithms can be
        initialised with None as solution and None as termination_criterion.
    stop_criterion : AbstractTerminationCriterion
        The termination criterion that will be used for all combinations of
        algorithms and problems.
    runs : int, optional
        The amount of runs that will be performed for a single
        algorithm-problem pair.
    seeds : list of int or tuple of int
        The seeds that will be used in the runs. Note that the length of the
        tuple or array needs to be equal to the amount of runs. If no seeds are
        given the seeds will be the number of the run.

    Returns
    -------
    list of list of list of namedtuple
        A 3-dimensional list of namedtuple. These namedtuples are the results
        of the algorithms. The first indice represents an algorithm, the second
        a problem, the third a run of the algorithm-problem pair. The indices
        that should be used are the same as in algorithms and solutions
        respectively for the first 2 indices. The third indice is used to
        choose between the runs. The possible indices for runs are always in
        the interval [0, #runs-1].

    """

    # set seeds if needed
    if seeds is None:
        seeds = range(runs)

    results = []

    algorithm_number = 0
    problem_number = 0
    seed_number = 0

    if(log):
        print('____Benchmark started___')
    # run everything
    for algorithm in algorithms:

        results_single_algorithm = []
        if(log):
            print('|---  Starting runs for algorithm ' + str(algorithm_number))

        for problem in problems:

            # setting problems and termination criterions
            algorithm._problem = problem
            if stop_criterion is not None:
                algorithm._termination_criterion = stop_criterion

            different_seed_results = []
            if(log):
                print('--|---  Starting runs for problem ' + str(problem_number))

            for i in seeds:
                if(log):
                    print('----|---  Starting run for seed ' + str(i))
                seed(i)
                algorithm.reset()
                different_seed_results.append(algorithm.run())
                if(log):
                    print('----|--- Completed run for seed ' + str(i))
                seed_number += 1

            results_single_algorithm.append(different_seed_results)

            if(log):
                print('--|--- Completed runs for problem ' + str(problem_number))
            problem_number += 1

        results.append(results_single_algorithm)
        if(log):
            print('|--- Completed runs for algorithm ' + str(algorithm_number))
        algorithm_number += 1

    if(log):
        print('____Benchmark ended___')

    return results
