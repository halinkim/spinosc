import numpy as np
import matplotlib.pyplot as plt

dt = 0.01
t = 0
N = 10000
T = dt * N

omega = 0.5
Omega_0 = 0.5
Omega_1 = 0.05
Delta = omega - Omega_0

Omega = (Delta ** 2 + Omega_1 ** 2) ** .5
omega_1 = Omega / Omega_1
omega_2 = Delta / Omega_1

t = np.linspace(0, 1 * np.pi / Omega, N + 1)

a_t = ((omega_1 + omega_2) / 2 / omega_1 * np.exp(1j / 2 * t * Omega) 
       + (omega_1 - omega_2) / 2 / omega_1 * np.exp(-1j / 2 * t * Omega))
b_t = (-(omega_1 - omega_2) * (omega_1 + omega_2) / 2 / omega_1 * np.exp(1j / 2 * t * Omega) 
       + (omega_1 + omega_2) * (omega_1 - omega_2) / 2 / omega_1 * np.exp(-1j / 2 * t * Omega))

A_t = a_t * np.exp(-1j / 2 * omega * t)
B_t = b_t * np.exp(1j / 2 * t * omega)

S_x = (np.conj(A_t) * B_t + np.conj(B_t) * A_t).real
S_y = (-np.conj(A_t) * B_t * 1j + np.conj(B_t) * A_t * 1j).real
S_z = (np.conj(A_t) * A_t - np.conj(B_t) * B_t).real

fig = plt.figure(dpi=300)
ax = fig.add_subplot(111, projection='3d')
cutLeft = 0
cutRight = cutLeft + 100000

ax.plot(S_x[cutLeft:cutRight], S_y[cutLeft:cutRight], S_z[cutLeft:cutRight])
ax.axis('equal')
ax.set_xlim(-1, 1)
ax.set_ylim(-1, 1)
ax.set_zlim(-1, 1)
plt.show()
