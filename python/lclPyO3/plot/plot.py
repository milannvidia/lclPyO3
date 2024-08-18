import matplotlib.pyplot as plt

def plot(data, title='Time - evaluation value plot.', width=20, height=10):
    """Plots the data in a time-value plot.

    Parameters
    ----------
    data : collections.namedtuple
        The data tuple one gets by performing a localsearch algorithm.
    title : str, optional
        The title of the plot. \n
        Default is 'Time - evaluation value plot.'.
    width : int, optional
        The width of the plot.
    height : int, optional
        The height of the plot.

    """

    # get the data from the data object
    time = data.time
    values = data.value

    plt.figure(figsize=(width, height))
    plt.plot(time, values, label='value')

    if hasattr(data, 'best_value'):
        best_values = data.best_value

        plt.plot(time, best_values, label='best value found')

    plt.xlabel('time (s)')
    plt.ylabel('evaluation value')

    plt.title(title)

    plt.legend()

    plt.show()


def iterations_plot(data, title='Iterations - evaluation value plot.',
                    width=20, height=10):
    """Plots the data in an iterations-value plot.

    Parameters
    ----------
    data : collections.namedtuple
        The data tuple one gets by performing a localsearch algorithm.
    title : str, optional
        The title of the plot. \n
        Default is 'Iterations - evaluation value plot.'.
    width : int, optional
        The width of the plot.
    height : int, optional
        The height of the plot.

    """

    # get the data from the data object
    iterations = data.iteration
    values = data.value

    plt.figure(figsize=(width, height))
    plt.plot(iterations, values, label='value')

    if hasattr(data, 'best_value'):
        best_values = data.best_value

        plt.plot(iterations, best_values, label='best value found')

    plt.xlabel('iterations')
    plt.ylabel('evaluation value')

    plt.title(title)

    plt.legend()

    plt.show()


def plot_single_stat(benchmark_single_stat, title='Single stat table',
                     algorithm_names=None, problem_names=None, width=10,
                     height=None):
    """Plots a single statistic of a benchmark.

    Parameters
    ----------
    benchmark_single_stat : numpy.ndarray
        A single 2D array that is the result from a function in
        "lclPyO3.benchmark.statistics".
    algorithm_names : list of str or tuple of str, optional
        Must be the same length as the algorithm list given to the original
        benchmark. The indices of a name corresponds with the indices of the
        algorithms given to the original benchmark.
    problem_names : list of str or tuple of str, optional
        Must be the same length as the problem list given to the original
        benchmark. The indices of a name corresponds with the indices of the
        problems given to the original benchmark.
    width : float, optional
        The width of the table in inches. Default is 10.
    height : float, optional
        The height of the table in inches. The default scales with the amount
        of items in the table.

    """

    if algorithm_names is None:
        algorithm_names = tuple(range(len(benchmark_single_stat)))

    if problem_names is None:
        problem_names = tuple(range(len(benchmark_single_stat[0])))

    fig, ax = plt.subplots()

    ax.axis('off')

    fig.set_figwidth(width)

    if height is None:
        vert_size = len(benchmark_single_stat)
        fig.set_figheight(vert_size * 0.2 + 1.2)
    else:
        fig.set_figheight(height)

    table = ax.table(cellText=benchmark_single_stat, rowLabels=algorithm_names,
                     colLabels=problem_names, loc='center')

    table.set_fontsize(14)
    table.scale(1.5, 1.5)

    plt.title(title, fontsize=20)
    plt.show()

def plotRust(data, title='Time - evaluation value plot.', width=20, height=10):
    """Plots the data in a time-value plot.

    Parameters
    ----------
    data : a list of tuples where the data represents the following (time(ns),best_found,score,iterations)
    title : str, optional
        The title of the plot. \n
        Default is 'Time - evaluation value plot.'.
    width : int, optional
        The width of the plot.
    height : int, optional
        The height of the plot.

    """

    # get the data from the data object
    time = [t[0]/(10**9) for t in data]
    values = [t[2] for t in data]
    best_values = [t[1] for t in data]

    plt.figure(figsize=(width, height))

    plt.plot(time, values, label='value')
    plt.plot(time, best_values, label='best value found')

    plt.xlabel('time (s)')
    plt.ylabel('evaluation value')

    plt.title(title)

    plt.legend()

    plt.show()

def iterations_plotRust(data, title='Iterations - evaluation value plot.',
                    width=20, height=10):
    """Plots the data in an iterations-value plot.

    Parameters
    ----------
    data : collections.namedtuple
        The data tuple one gets by performing a localsearch algorithm.
    title : str, optional
        The title of the plot. \n
        Default is 'Iterations - evaluation value plot.'.
    width : int, optional
        The width of the plot.
    height : int, optional
        The height of the plot.

    """

    # get the data from the data object
    iterations = [t[3] for t in data]
    values = [t[2] for t in data]
    best_values = [t[1] for t in data]

    plt.figure(figsize=(width, height))
    plt.plot(iterations, values, label='value')

    if hasattr(data, 'best_value'):
        best_values = data.best_value

        plt.plot(iterations, best_values, label='best value found')

    plt.xlabel('iterations')
    plt.ylabel('evaluation value')

    plt.title(title)

    plt.legend()

    plt.show()

def plotPythonRust(pythonData,rustData,title='Time - evaluation value plot Python Rust.', width=20, height=10):
    # get the data from the data object
    rTime = [t[0]/(10**9) for t in rustData]
    rValues = [t[2] for t in rustData]
    rBest_values = [t[1] for t in rustData]
    
    f, (ax1, ax2) = plt.subplots(1, 2, sharey=True,figsize=(width, height))
    ax2.plot(rTime,rValues,label='value')
    ax2.plot(rTime,rBest_values,label='best value')

    time = pythonData.time
    values = pythonData.value
    ax1.plot(time,values,label='value')
    if hasattr(pythonData, 'best_value'):
        best_values = pythonData.best_value
        ax1.plot(time,best_values,label='best value')

    plt.ylabel('evaluation value')
    ax1.set_xlabel('time (s)')
    ax2.set_xlabel('time (s)')
    ax1.set_title('Python')
    ax2.set_title('Rust')