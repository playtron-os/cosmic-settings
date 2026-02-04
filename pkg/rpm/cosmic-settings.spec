Name:           cosmic-settings
Epoch:          1
Version: 1.0.1
Release:        1%{?dist}
Summary:        COSMIC Settings - System settings application for COSMIC desktop (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-settings
Source0:        %{name}-%{_arch}.tar.gz

# No BuildRequires - binary is pre-built

Requires:       accountsservice
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
%autosetup -n %{name} -p1

%build

%install
# Binary
install -Dm0755 "usr/bin/cosmic-settings" "%{buildroot}%{_bindir}/cosmic-settings"

# Desktop files
cd "usr/share/applications" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/applications/{}" \;
cd -

# Icons
cd "usr/share/icons/hicolor" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/icons/hicolor/{}" \;
cd -

# Default schema (COSMIC configuration defaults)
cd "usr/share/cosmic" && find * -type f -exec install -Dm0644 '{}' "%{buildroot}%{_datadir}/cosmic/{}" \;
cd -

# Metainfo
install -Dm0644 "usr/share/metainfo/com.system76.CosmicSettings.metainfo.xml" "%{buildroot}%{_datadir}/metainfo/com.system76.CosmicSettings.metainfo.xml"

# Polkit policy and rules
install -Dm0644 "usr/share/polkit-1/actions/com.system76.CosmicSettings.Users.policy" "%{buildroot}%{_datadir}/polkit-1/actions/com.system76.CosmicSettings.Users.policy"
install -Dm0644 "usr/share/polkit-1/rules.d/cosmic-settings.rules" "%{buildroot}%{_datadir}/polkit-1/rules.d/cosmic-settings.rules"

# License
install -Dm0644 "usr/share/licenses/cosmic-settings/LICENSE.md" "%{buildroot}%{_datadir}/licenses/cosmic-settings/LICENSE.md"

%files
%license %{_datadir}/licenses/cosmic-settings/LICENSE.md
%{_bindir}/cosmic-settings
%{_datadir}/applications/com.system76.CosmicSettings*.desktop
%{_datadir}/icons/hicolor/*/apps/com.system76.CosmicSettings.svg
%{_datadir}/icons/hicolor/*/status/*.svg
%{_datadir}/cosmic/*
%{_datadir}/metainfo/com.system76.CosmicSettings.metainfo.xml
%{_datadir}/polkit-1/actions/com.system76.CosmicSettings.Users.policy
%{_datadir}/polkit-1/rules.d/cosmic-settings.rules

%changelog
* Tue Feb 03 2026 Playtron <dev@playtron.one> - 1.0.5-1
- Initial RPM package for Playtron fork
