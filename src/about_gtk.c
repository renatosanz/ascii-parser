#include "about_gtk.h"
#include "gdk/gdk.h"
#include "gtk/gtk.h"

void display_about_dialog() {
  GError *error = NULL;

  // Load texture from resource
  GdkTexture *logo =
      gdk_texture_new_from_resource("/org/asciiparser/data/icons/logo.png");

  // Check if the texture was loaded successfully
  if (!logo) {
    g_printerr("Failed to load logo from resource\n");
    // Fallback to icon name if resource fails
  }

  GtkWidget *about = gtk_about_dialog_new();

  // Set the parent window
  gtk_window_set_modal(GTK_WINDOW(about), TRUE);

  // Set program name
  gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(about), "ascii-parser");

  // Set version
  gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(about), "0.3");

  // Set copyright
  gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(about),
                                 "CC @ 2025 Renato Sanchez");

  // Set comments (short description)
  gtk_about_dialog_set_comments(
      GTK_ABOUT_DIALOG(about),
      "A image parser that converts images to ASCII characters, capable of "
      "generating both text files and rendered PNG images.");

  // Set license
  gtk_about_dialog_set_license_type(GTK_ABOUT_DIALOG(about),
                                    GTK_LICENSE_ARTISTIC);

  // Set website
  gtk_about_dialog_set_website(
      GTK_ABOUT_DIALOG(about),
      "https://riprtx.netlify.app/blogs/ascii-parser-blog/");
  gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(about), "Visit my blog");

  // Set authors
  const char *authors[] = {"Renato Sanchez <renato.sanchez3103@gmail.com>",
                           NULL};
  gtk_about_dialog_set_authors(GTK_ABOUT_DIALOG(about), authors);

  // const char *documenters[] = {"Doc Writer <doc@example.com>", NULL};
  // gtk_about_dialog_set_documenters(GTK_ABOUT_DIALOG(about), documenters);

  // const char *artists[] = {"Artist Name <artist@example.com>", NULL};
  // gtk_about_dialog_set_artists(GTK_ABOUT_DIALOG(about), artists);

  // gtk_about_dialog_set_translator_credits(GTK_ABOUT_DIALOG(about),
  //                                         "Translator 1\nTranslator 2");

  // Set logo - Cast GdkTexture to GdkPaintable
  if (logo) {
    gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(about), GDK_PAINTABLE(logo));
    g_object_unref(logo); // Unref after setting
  }

  gtk_window_present(GTK_WINDOW(about));
}
