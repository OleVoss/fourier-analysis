import numpy as np
import matplotlib.pyplot as plt
import scipy.interpolate
import scipy.integrate
import pprint


global NUM_COEF
NUM_COEF = 5

def interpolation(X_list,a_list): 
    A1=scipy.interpolate.UnivariateSpline(
        np.asarray(X_list),
        np.asarray(a_list),
        k=3,
        s=0,
        check_finite=True)

    m=(((a_list[-1])-(a_list[-2]))
       / ((X_list[-1])-(X_list[-2])))

    # A1ext= m*X+a_list[-1]-m*X_list[-1]

    # return lambda X: A1 if X_list[-1] >= X else A1ext
    return A1

def integrate_a(a, p):
    coef_array = []
    for k in range(0, NUM_COEF+1):
        result = scipy.integrate.quad(
            lambda x: a(x) * np.cos(((2*np.pi)/p)*k*x),
            0,
            p
        )
        a_k = 2/p * result[0]
        coef_array.append(a_k)
    return coef_array

def integrate_b(a, p):
    coef_array = []
    for k in range(0, NUM_COEF+1):
        result = scipy.integrate.quad(
            lambda x: a(x) * np.sin(((2*np.pi)/p)*k*x),
            0,
            p
        )
        a_k = 2/p * result[0]
        coef_array.append(a_k)
    return coef_array

# data
x = np.array([1, 0.75, 0.5, 0, -0.5, -1, -0.5, 0, 0.5, 1])
y = np.array([0, 0.5, 1, 0.5, 0, -0.5, -1, -0.5, 0])

t_x = np.arange(0, len(x), 1)
t = np.array([0,1,2,3,4,5,6,7,8])

x_new = np.arange(0, 8, .1)
y_new = np.zeros(80)

a = interpolation(t_x, x)

# plotting
for i, xi in enumerate(x_new):
    y_new[i] = a(xi)

plt.figure(figsize = (12, 8))
# plt.plot(t_x, x, 'bo')
plt.plot(t_x, x, "bo", color="orange")
# plt.plot(x_new, y_new_x)
plt.plot(x_new, y_new)
plt.show()

pp = pprint.PrettyPrinter(indent=4)
pp.pprint(integrate_a(a, len(x)))
pp.pprint(integrate_b(a, len(x)))