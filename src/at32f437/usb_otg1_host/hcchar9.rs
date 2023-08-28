#[doc = "Register `HCCHAR9` reader"]
pub type R = crate::R<HCCHAR9_SPEC>;
#[doc = "Register `HCCHAR9` writer"]
pub type W = crate::W<HCCHAR9_SPEC>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `EPTNUM` reader - Endpoint number"]
pub type EPTNUM_R = crate::FieldReader;
#[doc = "Field `EPTNUM` writer - Endpoint number"]
pub type EPTNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EPTDIR` reader - Endpoint direction"]
pub type EPTDIR_R = crate::BitReader;
#[doc = "Field `EPTDIR` writer - Endpoint direction"]
pub type EPTDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSPDDEV` reader - Low-speed device"]
pub type LSPDDEV_R = crate::BitReader;
#[doc = "Field `LSPDDEV` writer - Low-speed device"]
pub type LSPDDEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MC` reader - Multicount"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multicount"]
pub type MC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DEVADDR` reader - Device address"]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - Device address"]
pub type DEVADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type CHDIS_R = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type CHDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type CHENA_R = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type CHENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<HCCHAR9_SPEC, 0> {
        MPS_W::new(self)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn eptnum(&mut self) -> EPTNUM_W<HCCHAR9_SPEC, 11> {
        EPTNUM_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn eptdir(&mut self) -> EPTDIR_W<HCCHAR9_SPEC, 15> {
        EPTDIR_W::new(self)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lspddev(&mut self) -> LSPDDEV_W<HCCHAR9_SPEC, 17> {
        LSPDDEV_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HCCHAR9_SPEC, 18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<HCCHAR9_SPEC, 20> {
        MC_W::new(self)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<HCCHAR9_SPEC, 22> {
        DEVADDR_W::new(self)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HCCHAR9_SPEC, 29> {
        ODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<HCCHAR9_SPEC, 30> {
        CHDIS_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<HCCHAR9_SPEC, 31> {
        CHENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS host channel-9 characteristics register (OTGFS_HCCHAR9)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCCHAR9_SPEC;
impl crate::RegisterSpec for HCCHAR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcchar9::R`](R) reader structure"]
impl crate::Readable for HCCHAR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcchar9::W`](W) writer structure"]
impl crate::Writable for HCCHAR9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR9 to value 0"]
impl crate::Resettable for HCCHAR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
