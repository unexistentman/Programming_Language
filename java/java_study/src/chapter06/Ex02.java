package chapter06;

class Point1 {
	int x, y;
	public Point1(int x, int y) {
		this.x = x;
		this.y = y;
	}
	public String toString() {
		return "Point(" + x + "," + y + ")";
	}
}

public class Ex02 {

	public static void main(String[] args) {
		Point1 p = new Point1(2, 3);
		System.out.println(p.toString());
		System.out.println(p);
		System.out.println(p + "�Դϴ�.");

	}

}
