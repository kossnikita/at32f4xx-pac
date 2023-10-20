#[doc = "Register `HCCHAR13` reader"]
pub type R = crate::R<HCCHAR13_SPEC>;
#[doc = "Register `HCCHAR13` writer"]
pub type W = crate::W<HCCHAR13_SPEC>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPTNUM` reader - Endpoint number"]
pub type EPTNUM_R = crate::FieldReader;
#[doc = "Field `EPTNUM` writer - Endpoint number"]
pub type EPTNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPTDIR` reader - Endpoint direction"]
pub type EPTDIR_R = crate::BitReader;
#[doc = "Field `EPTDIR` writer - Endpoint direction"]
pub type EPTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPDDEV` reader - Low-speed device"]
pub type LSPDDEV_R = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-speed device"]
pub type LSPDDEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MC` reader - Multicount"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multicount"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type CHDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type CHENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn eptnum(&self) -> EPTNUM_R {
        EPTNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn eptdir(&self) -> EPTDIR_R {
        EPTDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lspddev(&self) -> LSPDDEV_R {
        LSPDDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR13")
            .field("mps", &format_args!("{}", self.mps().bits()))
            .field("eptnum", &format_args!("{}", self.eptnum().bits()))
            .field("eptdir", &format_args!("{}", self.eptdir().bit()))
            .field("lspddev", &format_args!("{}", self.lspddev().bit()))
            .field("eptype", &format_args!("{}", self.eptype().bits()))
            .field("mc", &format_args!("{}", self.mc().bits()))
            .field("devaddr", &format_args!("{}", self.devaddr().bits()))
            .field("oddfrm", &format_args!("{}", self.oddfrm().bit()))
            .field("chdis", &format_args!("{}", self.chdis().bit()))
            .field("chena", &format_args!("{}", self.chena().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCCHAR13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<HCCHAR13_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn eptnum(&mut self) -> EPTNUM_W<HCCHAR13_SPEC> {
        EPTNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn eptdir(&mut self) -> EPTDIR_W<HCCHAR13_SPEC> {
        EPTDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lspddev(&mut self) -> LSPDDEV_W<HCCHAR13_SPEC> {
        LSPDDEV_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HCCHAR13_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<HCCHAR13_SPEC> {
        MC_W::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<HCCHAR13_SPEC> {
        DEVADDR_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HCCHAR13_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<HCCHAR13_SPEC> {
        CHDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<HCCHAR13_SPEC> {
        CHENA_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGHS host channel-13 characteristics register (OTGHS_HCCHAR13)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR13_SPEC;
impl crate::RegisterSpec for HCCHAR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar13::R`](R) reader structure"]
impl crate::Readable for HCCHAR13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar13::W`](W) writer structure"]
impl crate::Writable for HCCHAR13_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR13 to value 0"]
impl crate::Resettable for HCCHAR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
