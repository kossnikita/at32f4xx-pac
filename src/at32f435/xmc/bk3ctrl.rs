#[doc = "Register `BK3CTRL` reader"]
pub type R = crate::R<BK3CTRL_SPEC>;
#[doc = "Register `BK3CTRL` writer"]
pub type W = crate::W<BK3CTRL_SPEC>;
#[doc = "Field `NWEN` reader - Wait feature enable"]
pub type NWEN_R = crate::BitReader;
#[doc = "Field `NWEN` writer - Wait feature enable"]
pub type NWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEV` reader - Memory device type"]
pub type DEV_R = crate::BitReader;
#[doc = "Field `DEV` writer - Memory device type"]
pub type DEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTMDBW` reader - External memory data bus width"]
pub type EXTMDBW_R = crate::FieldReader;
#[doc = "Field `EXTMDBW` writer - External memory data bus width"]
pub type EXTMDBW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ECCEN` reader - ECC enable"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC enable"]
pub type ECCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCR` reader - CLE to RE delay"]
pub type TCR_R = crate::FieldReader;
#[doc = "Field `TCR` writer - CLE to RE delay"]
pub type TCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TAR` reader - ALE to RE delay"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to RE delay"]
pub type TAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ECCPGS` reader - ECC page size"]
pub type ECCPGS_R = crate::FieldReader;
#[doc = "Field `ECCPGS` writer - ECC page size"]
pub type ECCPGS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&self) -> NWEN_R {
        NWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&self) -> DEV_R {
        DEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&self) -> EXTMDBW_R {
        EXTMDBW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    pub fn eccpgs(&self) -> ECCPGS_R {
        ECCPGS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwen(&mut self) -> NWEN_W<BK3CTRL_SPEC, 1> {
        NWEN_W::new(self)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BK3CTRL_SPEC, 2> {
        EN_W::new(self)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    #[must_use]
    pub fn dev(&mut self) -> DEV_W<BK3CTRL_SPEC, 3> {
        DEV_W::new(self)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn extmdbw(&mut self) -> EXTMDBW_W<BK3CTRL_SPEC, 4> {
        EXTMDBW_W::new(self)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<BK3CTRL_SPEC, 6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<BK3CTRL_SPEC, 9> {
        TCR_W::new(self)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<BK3CTRL_SPEC, 13> {
        TAR_W::new(self)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    #[must_use]
    pub fn eccpgs(&mut self) -> ECCPGS_W<BK3CTRL_SPEC, 17> {
        ECCPGS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK3CTRL_SPEC;
impl crate::RegisterSpec for BK3CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3ctrl::R`](R) reader structure"]
impl crate::Readable for BK3CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk3ctrl::W`](W) writer structure"]
impl crate::Writable for BK3CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK3CTRL to value 0x18"]
impl crate::Resettable for BK3CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
