package chapter14;
import javax.sound.sampled.*;
import java.awt.*;
import javax.swing.*;
import java.io.*;

public class Ex13 extends JFrame {
	private Clip clip;
	private String song = "audio/�ֱ���1��.wav";
	private JLabel label = new JLabel(song);
	
	public Ex13() {
		setTitle("�ֱ��� 1�� ����");
		setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		Container c = getContentPane();
		c.setLayout(new FlowLayout());
		c.setBackground(Color.YELLOW);
		c.add(label);
		setSize(300,150);
		setVisible(true);
		loadAudio(song);
	}
	
	private void loadAudio(String pathName) {
		try {
			File audioFile = new File(pathName);
			final AudioInputStream audioStream = AudioSystem.getAudioInputStream(audioFile);
			clip = AudioSystem.getClip();
			clip.addLineListener(new LineListener() {
				public void update(LineEvent e) {
					if(e.getType() == LineEvent.Type.STOP) {
						try {
							getContentPane().setBackground(Color.ORANGE);
							label.setText(song + " ���� ��!");
							audioStream.close();
						} catch (IOException e1) {
							e1.printStackTrace();
						}
					}
				}
			});
			
			clip.open(audioStream);
			clip.start();
		}
		catch (LineUnavailableException e) { e.printStackTrace(); }
		catch (UnsupportedAudioFileException e) { e.printStackTrace(); }
		catch (IOException e) {e.printStackTrace(); }
	}

	public static void main(String[] args) {
		new Ex13();

	}

}