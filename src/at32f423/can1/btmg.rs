#[doc = "Register `BTMG` reader"]
pub type R = crate::R<BTMG_SPEC>;
#[doc = "Register `BTMG` writer"]
pub type W = crate::W<BTMG_SPEC>;
#[doc = "Field `BRDIV` reader - Baud rate division"]
pub type BRDIV_R = crate::FieldReader<u16>;
#[doc = "Field `BRDIV` writer - Baud rate division"]
pub type BRDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `BTS1` reader - Bit time segment 1"]
pub type BTS1_R = crate::FieldReader;
#[doc = "Field `BTS1` writer - Bit time segment 1"]
pub type BTS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BTS2` reader - Bit time segment 2"]
pub type BTS2_R = crate::FieldReader;
#[doc = "Field `BTS2` writer - Bit time segment 2"]
pub type BTS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RSAW` reader - Resynchronization adjust width"]
pub type RSAW_R = crate::FieldReader;
#[doc = "Field `RSAW` writer - Resynchronization adjust width"]
pub type RSAW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LBEN` reader - Loop back mode"]
pub type LBEN_R = crate::BitReader;
#[doc = "Field `LBEN` writer - Loop back mode"]
pub type LBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOEN` reader - Listen-Only mode"]
pub type LOEN_R = crate::BitReader;
#[doc = "Field `LOEN` writer - Listen-Only mode"]
pub type LOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    pub fn brdiv(&self) -> BRDIV_R {
        BRDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    pub fn bts1(&self) -> BTS1_R {
        BTS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    pub fn bts2(&self) -> BTS2_R {
        BTS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    pub fn rsaw(&self) -> RSAW_R {
        RSAW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    pub fn lben(&self) -> LBEN_R {
        LBEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    pub fn loen(&self) -> LOEN_R {
        LOEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    #[must_use]
    pub fn brdiv(&mut self) -> BRDIV_W<BTMG_SPEC, 0> {
        BRDIV_W::new(self)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bts1(&mut self) -> BTS1_W<BTMG_SPEC, 16> {
        BTS1_W::new(self)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bts2(&mut self) -> BTS2_W<BTMG_SPEC, 20> {
        BTS2_W::new(self)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    #[must_use]
    pub fn rsaw(&mut self) -> RSAW_W<BTMG_SPEC, 24> {
        RSAW_W::new(self)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lben(&mut self) -> LBEN_W<BTMG_SPEC, 30> {
        LBEN_W::new(self)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    #[must_use]
    pub fn loen(&mut self) -> LOEN_W<BTMG_SPEC, 31> {
        LOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btmg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btmg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTMG_SPEC;
impl crate::RegisterSpec for BTMG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btmg::R`](R) reader structure"]
impl crate::Readable for BTMG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btmg::W`](W) writer structure"]
impl crate::Writable for BTMG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTMG to value 0"]
impl crate::Resettable for BTMG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
