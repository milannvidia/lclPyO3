import numpy as np

from lclPyO3.evaluation.abstract_evaluation_function \
    import AbstractEvaluationFunction
from lclPyO3.evaluation.deltaeval.delta_eval_func import delta_eval_func


class binProblemEval(AbstractEvaluationFunction):
    def __init__(self, size_bin,sizes):
        super().__init__()
        self._size_bin = size_bin
        self._sizes=sizes


    def get_problem_type(self):
        return 'bin'

    def evaluate(self, order):

        value = 1

        # add all distances to value
        currentBinSize=0
        for x in order:
            if(self._sizes[x]+currentBinSize>self._size_bin):
                value +=1
                currentBinSize=self._sizes[x]
            else:
                currentBinSize+=self._sizes[x]
        return value

    def delta_evaluate(self, current_data, move):
        currentScore=self.evaluate(current_data)
        moveArray=np.array(current_data)

        moveArray[move[0]],moveArray[move[1]]=moveArray[move[1]],moveArray[move[0]]
        moveScore=self.evaluate(moveArray)

        return moveScore-currentScore
