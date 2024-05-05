# Flutter Mobile Template

This repository contains a Flutter project that serves as a template for mobile application development. This template is designed to streamline the process of creating a new Flutter application with pre-installed toaster packages, a clean project structure, and a simple README file to guide new developers. the template include a setup for all the necessary tools to start a new project.

> Note: All the instructions in this README file are for linux users. If you are using another operating system, you may need to adapt the commands.

## Used tools

-   **Flutter**: Open-source UI software development kit created by Google.
-   **Dart**: Programming language optimized for building mobile, desktop, server, and web applications.
-   **Android Studio**: Integrated development environment for Android app development.
-   **Visual Studio Code**: Code editor optimized for building and debugging modern web and cloud applications.
-   **Git**: Version control system used to track changes in the source code.

## Setup Flutter

To set up Flutter, the steps are get of the [official documentation](https://docs.flutter.dev/get-started/install/linux/android?tab=download):

> Used flutter version: 3.19.5 (channel stable) - 28/03/2024

### 1. Download Flutter SDK

Go to downloads folder

```bash
cd ~/Downloads
```

Download the Flutter SDK using the following command:

```bash
wget https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.19.5-stable.tar.xz
```

### 2. Extract the Flutter SDK

Extract the downloaded file using the following command:

```bash
tar xf flutter_linux_3.19.5-stable.tar.xz
```

### 3. Move the Flutter SDK to the desired location

Now, move the extracted Flutter SDK to the desired location. In this example, we will move it to `/usr/local`:

```bash
sudo mv flutter /usr/local
```

### 4. Add the Flutter SDK to your path

You can add the following line to your `.bashrc` or `.zshrc` file:

```bash
export PATH="$PATH:/usr/local/flutter/bin"
```

### 5. Test the Flutter installation

To test the Flutter installation, run the following command:

```bash
flutter --version
```

Expected output:

```bash
Flutter 3.19.5 • channel stable • https://github.com/flutter/flutter.git
Framework • revision 300451adae (3 weeks ago) • 2024-03-27 21:54:07 -0500
Engine • revision e76c956498
Tools • Dart 3.3.3 • DevTools 2.31.1
```

If you have any issues, refer to the [official documentation](https://docs.flutter.dev/get-started/install/linux/android?tab=download) for more detailed instructions.

## Setup Android Studio

Now that you have Flutter installed, you need to set up Android Studio. The steps are get of the [official documentation](https://developer.android.com/studio/install#linux):

> Used Android Studio version: 2023.2.1.25

### 1. Download Android Studio

```bash
wget https://redirector.gvt1.com/edgedl/android/studio/ide-zips/2023.2.1.25/android-studio-2023.2.1.25-linux.tar.gz
```

### 2. Extract the Android Studio archive

Extract the downloaded file using the following command:

```bash
tar xf android-studio-2023.2.1.25-linux.tar.gz
```

### 3. Move the Android Studio folder to the desired location

Now, move the extracted Android Studio folder to the desired location. In this example, we will move it to `/usr/local`:

```bash
sudo mv android-studio /usr/local
```

### 4. Create a desktop entry

Desktop entries are used to create shortcuts in the application menu. To create a desktop entry for Android Studio, run the following command:

```bash
echo -e "[Desktop Entry]\nName=Android Studio\nComment=Android Studio IDE\nExec=/usr/local/android-studio/bin/studio.sh\nIcon=/usr/local/android-studio/bin/studio.png\nTerminal=false\nType=Application\nCategories=Development;IDE;Java;" | sudo tee /usr/share/applications/android-studio.desktop
```

### 5. Test the Android Studio installation

You can find Android Studio in the application menu and launch it from there, if the icon is not there, you can log out and log in again to refresh the application menu. Or you can run the following command:

```bash
/usr/local/android-studio/bin/studio.sh
```

If you have any issues, refer to the [official documentation](https://developer.android.com/studio/install#linux) for more detailed instructions.

## Setup Project

To set up this project, follow these steps:

### 1. Clone the repository

First, clone the repository using Git:

```bash
git clone git@github.com:ViniciosLugli/flutter-mobile-template.git
```

and navigate to the project directory:

```bash
cd flutter-mobile-template
```

### 2. Install dependencies

Run the following command in the terminal at the project root:

```bash
flutter pub get
```

This command retrieves all the necessary Flutter dependencies.

### 3. Open the project

Open the project in your IDE:

-   Open Visual Studio Code.
-   Click on "Open Folder".
-   Navigate to the project directory and select it.

### 4. Install packages on Android Studio

Now to install the command line tools, open Android Studio and follow these steps:

-   Open Android Studio.
-   Click on "Configure" in the bottom right corner.
-   Select "SDK Manager".
-   Click on the "SDK Tools" tab.
-   Check the "Android SDK Command-line Tools" box.
-   Click "Apply" to install the tools.

### 5. Create a mobile emulator

Create a mobile emulator using Android Studio:

-   Open Android Studio.
-   Click on "Configure" in the bottom right corner.
-   Select "AVD Manager".
-   Click on "Create Virtual Device".
-   Choose a device and click "Next".
-   Select a system image and click "Next".
-   Click "Finish" to create the emulator.

### 6. Configure flutter device

Add android sdk to your path, by default the android sdk is installed in the home directory, if you have installed in another directory, change the path.

```bash
flutter config --android-sdk $HOME/Android/Sdk
```

Run the following command in the terminal to list the available emulators devices:

```bash
flutter emulators
```

Copy the device name of the android emulator you created.

Run the following command to configure the device:

```bash
flutter emulators --launch <device_name>
```

### 7. Run the project

To run the project, use the following command in your terminal:

```bash
flutter run
```

Now you should see the app running on the emulator. Make sure you have an emulator running to see the app.

For more detailed instructions, refer to the [Flutter official documentation](https://flutter.dev/docs).

## Demo

![demo](https://github.com/ViniciosLugli/flutter-mobile-template/assets/40807526/3e592825-bcfc-4a55-a3cf-8ea35a724ad9)
