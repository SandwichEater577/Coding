with Ada.Text_IO; use Ada.Text_IO;

procedure Hello is
begin
   for i in 1 .. 5 loop
      Put_Line ("Step:" & Integer'Image (i));
   end loop;
end Hello;