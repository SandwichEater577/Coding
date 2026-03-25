using System;

namespace AboutMeApp
{
    class Program
    {
        static void Main()
        {
            string name = "Michal Kostkowski";
            DateTime birthDate = new DateTime(2010, 12, 13);
            DateTime today = DateTime.Today;

            int age = today.Year - birthDate.Year;
            if (birthDate.Date > today.AddYears(-age))
            {
                age--;
            }
            Console.WriteLine($"Czesc, mam na imie {name}.");
            Console.WriteLine($"Mam {age} lat.");
        }
    }
}