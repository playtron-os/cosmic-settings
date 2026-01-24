Name:           cosmic-settings
Epoch:          1
Version:        %{getenv:COSMIC_SETTINGS_VERSION}
Release:        1%{?dist}
Summary:        COSMIC Settings - System settings application for COSMIC desktop (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-settings

# No BuildRequires - binary is pre-built

Requires:       dbus
Requires:       polkit

# Override the upstream cosmic-settings
Provides:       cosmic-settings = %{epoch}:%{version}-%{release}
Obsoletes:      cosmic-settings < %{epoch}:%{version}

%description
System settings application for the COSMIC desktop environment.
Provides a graphical interface for configuring system settings including
display, input, network, sound, appearance, and more.

%prep
%build

%install
# COSMIC_SETTINGS_SOURCE is set by the build script

# Binary
install -Dm0755 "%{getenv:COSMIC_SETTINGS_SOURCE}/target/release/cosmic-settings" "%{buildroot}%{_bindir}/cosmic-settings"

# Desktop files
cd "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/applications" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/applications/{}" \;

# Icons
cd "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/icons" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/icons/hicolor/{}" \;

# Default schema (COSMIC configuration defaults)
cd "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/default_schema" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/cosmic/{}" \;

# Metainfo
install -Dm0644 "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/com.system76.CosmicSettings.metainfo.xml" "%{buildroot}%{_datadir}/metainfo/com.system76.CosmicSettings.metainfo.xml"

# Polkit policy and rules
install -Dm0644 "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/polkit-1/actions/com.system76.CosmicSettings.Users.policy" "%{buildroot}%{_datadir}/polkit-1/actions/com.system76.CosmicSettings.Users.policy"
install -Dm0644 "%{getenv:COSMIC_SETTINGS_SOURCE}/resources/polkit-1/rules.d/cosmic-settings.rules" "%{buildroot}%{_datadir}/polkit-1/rules.d/cosmic-settings.rules"

# License
install -Dm0644 "%{getenv:COSMIC_SETTINGS_SOURCE}/LICENSE.md" "%{buildroot}%{_datadir}/licenses/cosmic-settings/LICENSE.md"

%files
%license %{_datadir}/licenses/cosmic-settings/LICENSE.md
%{_bindir}/cosmic-settings
%{_datadir}/applications/com.system76.CosmicSettings*.desktop
%{_datadir}/icons/hicolor/*/apps/com.system76.CosmicSettings.svg
%{_datadir}/cosmic/*
%{_datadir}/metainfo/com.system76.CosmicSettings.metainfo.xml
%{_datadir}/polkit-1/actions/com.system76.CosmicSettings.Users.policy
%{_datadir}/polkit-1/rules.d/cosmic-settings.rules

%changelog
* Wed Jan 21 2026 Playtron <dev@playtron.one> - 1.0.4-1
- Initial RPM package for Playtron fork
