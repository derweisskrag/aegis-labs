#include <cstddef>
#include <stdio.h>
#include <ctime>

int sum_gauss(int n){
    // For loop version
    int sum = 0;
    for (size_t i = 0; i < n; i++)
    {
        sum += i;
    }

    return sum;
}

int sum_gauss_formula(int n){
    // Formula version
    return (n * (n - 1)) / 2;
}

int main(){
    printf("Comparing the time taken by loop and formula versions of Gauss sum calculation...\n");
    int iterations = 100000000;
    int how_much_sum = 100;
    int sum1;
    int sum2;

    // evaluate the time taken by both functions
    int start_1 = clock();
    for (size_t i = 0; i < iterations; i++)
    {
        sum1 = sum_gauss(how_much_sum);
    }
    int end_1 = clock();
    double time_taken = (double)(end_1 - start_1) / CLOCKS_PER_SEC;
    printf("Time taken by loop version: %f seconds\n", time_taken);
    
    int start_2 = clock();
    for (size_t i = 0; i < iterations; i++)
    {
        sum2 = sum_gauss_formula(how_much_sum);
    }
    int end_2 = clock();
    double time_taken_formula = (double)(end_2 - start_2) / CLOCKS_PER_SEC;
    printf("Time taken by formula version: %f seconds\n", time_taken_formula);


    printf("Sum (loop): %d\n", sum1);
    printf("Sum (formula): %d\n", sum2);

    return 0;
}