package chapter02;
import java.util.Scanner;

public class Tr07 {

	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);
		
		int x, y = 0;
		System.out.print("�� (x, y)�� ��ǥ�� �Է��Ͻÿ�>>");
		x = scanner.nextInt();
		y = scanner.nextInt();
		
		if ((x >= 100 && x <= 200) && (y >= 100 && y <= 200)) {
			System.out.println("(" + x + "," + y + ")" + "�� �簢�� �ȿ� �ֽ��ϴ�.");
		}
		
		else {
			System.out.println("(" + x + "," + y + ")" + "�� �簢�� �ȿ� �����ϴ�.");
		}

		scanner.close();
	}

}